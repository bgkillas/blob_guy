use crate::noita::pixel::NoitaPixelRun;
use std::{ffi::c_void, mem};
pub(crate) mod ntypes;
pub(crate) mod pixel;

pub(crate) struct ParticleWorldState {
    pub(crate) _world_ptr: *mut c_void,
    pub(crate) chunk_map_ptr: *mut c_void,
    pub(crate) material_list_ptr: *const c_void,

    pub(crate) runner: pixel::PixelRunner<pixel::RawPixel>,
}

impl ParticleWorldState {
    fn get_cell_raw(&self, x: i32, y: i32) -> Option<&ntypes::Cell> {
        let x = x as isize;
        let y = y as isize;
        let chunk_index = (((((y) >> 9) - 256) & 511) * 512 + ((((x) >> 9) - 256) & 511)) * 4;
        // Deref 1/3
        let chunk_arr = unsafe { self.chunk_map_ptr.offset(8).cast::<*const c_void>().read() };
        // Deref 2/3
        let chunk = unsafe { chunk_arr.offset(chunk_index).cast::<*const c_void>().read() };
        if chunk.is_null() {
            return None;
        }
        // Deref 3/3
        let pixel_array = unsafe { chunk.cast::<*const c_void>().read() };
        let pixel = unsafe { pixel_array.offset((((y & 511) << 9) | x & 511) * 4) };
        if pixel.is_null() {
            return None;
        }

        unsafe { pixel.cast::<*const ntypes::Cell>().read().as_ref() }
    }

    fn get_cell_material_id(&self, cell: &ntypes::Cell) -> u16 {
        let mat_ptr = cell.material_ptr();
        let offset = unsafe { mat_ptr.cast::<c_void>().offset_from(self.material_list_ptr) };
        (offset / ntypes::CELLDATA_SIZE) as u16
    }

    fn get_cell_type(&self, cell: &ntypes::Cell) -> Option<ntypes::CellType> {
        unsafe { Some(cell.material_ptr().as_ref()?.cell_type) }
    }

    pub(crate) unsafe fn encode_area(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        pixel_runs: &mut Vec<NoitaPixelRun>,
    ) -> usize {
        // Allow compiler to generate better code.
        assert_eq!(start_x % 128, 0);
        assert_eq!(start_y % 128, 0);
        assert!((end_x - start_x) <= 128);
        assert!((end_y - start_y) <= 128);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let mut raw_pixel = pixel::RawPixel {
                    material: 0,
                    flags: 0,
                };
                let cell = self.get_cell_raw(x, y);
                if let Some(cell) = cell {
                    let cell_type = self.get_cell_type(cell).unwrap_or(ntypes::CellType::None);
                    match cell_type {
                        ntypes::CellType::None => {}
                        // Nobody knows how box2d pixels work.
                        ntypes::CellType::Solid => {}
                        ntypes::CellType::Liquid => {
                            raw_pixel.material = self.get_cell_material_id(cell);
                            let cell: &ntypes::LiquidCell = unsafe { mem::transmute(cell) };
                            raw_pixel.flags = cell.is_static as u8;
                        }
                        ntypes::CellType::Gas | ntypes::CellType::Fire => {
                            raw_pixel.material = self.get_cell_material_id(cell);
                        }
                        // ???
                        _ => {}
                    }
                }
                self.runner.put_pixel(raw_pixel);
            }
        }

        let built_runner = self.runner.build();
        let runs = built_runner.len();
        for run in built_runner {
            pixel_runs.push(NoitaPixelRun {
                length: (run.length - 1) as u16,
                material: run.data.material,
                flags: run.data.flags,
            })
        }
        self.runner.clear();
        runs
    }
}
