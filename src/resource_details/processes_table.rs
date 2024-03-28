use core::fmt;

use iced::{widget::{button, checkbox, column, container, horizontal_space, pick_list, row, text, text_input}, Element, Length, Renderer, Theme};
use iced_table::table;

use crate::{styles::container::divider_background_1, ResourceDetailsMessage};

#[derive(Debug)]
pub struct TableColumn {
    pub kind: TableColumnKind,
    pub width: f32,
    pub resize_offset: Option<f32>,
}

impl TableColumn {
    pub fn new(kind: TableColumnKind) -> Self {
        let width = match kind {
            // TableColumnKind::Index => 60.0,
            // TableColumnKind::Category => 100.0,
            // TableColumnKind::Enabled => 155.0,
            // TableColumnKind::Notes => 400.0,
            // TableColumnKind::Delete => 100.0,
            _ => 100.0,
        };

        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

#[derive(Debug)]
pub enum TableColumnKind {
    Name,
    Cpu,
    Memory,
    DiskRead,
    DiskWritten,
    Action,
}

#[derive(Debug)]
pub struct TableRow {
    pub notes: String,
    pub is_enabled: bool,

}

impl TableRow {
    pub fn generate(index: usize) -> Self {
        let is_enabled = index % 5 < 4;

        Self {
            notes: String::new(),
            is_enabled,
        }
    }
}

impl<'a> table::Column<'a, ResourceDetailsMessage, Theme, Renderer> for TableColumn {
    type Row = TableRow;

    fn header(&'a self, _col_index: usize) -> Element<'a, ResourceDetailsMessage> {
        let content = match self.kind {
            TableColumnKind::Name => "Name",
            TableColumnKind::Cpu => "CPU",
            TableColumnKind::Memory => "Memory",
            TableColumnKind::DiskRead => "Disk Read",
            TableColumnKind::DiskWritten => "Disk Written",
            TableColumnKind::Action => "Action",
        };

        container(text(content)).height(24).center_y().into()
    }

    fn cell(
        &'a self,
        col_index: usize,
        row_index: usize,
        row: &'a Self::Row,
    ) -> Element<'a, ResourceDetailsMessage> {
        let content: Element<_> = match self.kind {
            TableColumnKind::Name => {
                row![
                    text("App name")
                ].into()
            }
            // TableColumnKind::Index => text(row_index).into(),
            // TableColumnKind::Category => pick_list(Category::ALL, Some(row.category), move |category| {
            //     ResourceDetailsMessage::Category(row_index, category)
            // })
            // .into(),
            // TableColumnKind::Enabled => checkbox("", row.is_enabled)
            //     .on_toggle(move |enabled| ResourceDetailsMessage::Enabled(row_index, enabled))
            //     .into(),
            // TableColumnKind::Notes => text_input("", &row.notes)
            //     .on_input(move |notes| ResourceDetailsMessage::Notes(row_index, notes))
            //     .width(Length::Fill)
            //     .into(),
            // TableColumnKind::Delete => button(text("Delete"))
            //     .on_press(ResourceDetailsMessage::Delete(row_index))
            //     .into(),
            _ => {
                text("default").into()
            }
        };

        let seperator = if col_index > 0 {
            container(row![])
            .style(divider_background_1())
            .width(Length::Fill)
            .height(1)
        } else {
            container(row![])
        };

        container(column![seperator, content])
            .width(Length::Fill)
            .height(32)
            .center_y()
            .into()
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}