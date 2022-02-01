// cellStyles
use super::CellStyle;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct CellStyles {
    cell_style: Vec<CellStyle>,
}
impl CellStyles {
    pub(crate) fn _get_cell_style(&self) -> &Vec<CellStyle> {
        &self.cell_style
    }

    pub(crate) fn _get_cell_style_mut(&mut self) -> &mut Vec<CellStyle> {
        &mut self.cell_style
    }

    pub(crate) fn set_cell_style(&mut self, value: CellStyle) -> &mut Self {
        self.cell_style.push(value);
        self
    }

    pub(crate) fn get_defalut_value() -> CellStyle {
        let def = CellStyle::default();
        def
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        self.set_cell_style(Self::get_defalut_value());

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"xf" => {
                        let mut obj = CellStyle::default();
                        obj.set_attributes(reader, e);
                        self.set_cell_style(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"cellStyles" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "cellStyles"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.cell_style.len() > 0 {
            // cellStyles
            write_start_tag(
                writer,
                "cellStyles",
                vec![("count", &self.cell_style.len().to_string())],
                false,
            );

            // cellStyle
            for cell_style in &self.cell_style {
                cell_style.write_to(writer);
            }

            write_end_tag(writer, "cellStyles");
        }
    }
}
