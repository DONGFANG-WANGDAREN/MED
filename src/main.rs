use iced::widget::{button, column, container, row, scrollable, text, Column};
use iced::{Element, Length, Task, Theme};
use iced::window;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn main() -> iced::Result {
    iced::application(FileExplorer::default, FileExplorer::update, FileExplorer::view)
        .title("MED")
        .theme(|_state: &FileExplorer| Theme::Dark)
        .run()
}

struct FileExplorer {
    current_path: PathBuf,
    entries: Vec<FileEntry>,
    error_message: Option<String>,
}

#[derive(Debug, Clone)]
struct FileEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: String,
    modified: String,
}

#[derive(Debug, Clone)]
enum Message {
    NavigateTo(PathBuf),
    GoUp,
    OpenFile(PathBuf),
    Refresh,
}

impl Default for FileExplorer {
    fn default() -> Self {
        let current_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let (entries, error_message) = read_directory(&current_path);
        
        Self {
            current_path,
            entries,
            error_message,
        }
    }
}

impl FileExplorer {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(path) => {
                if path.is_dir() {
                    let (entries, error) = read_directory(&path);
                    self.current_path = path;
                    self.entries = entries;
                    self.error_message = error;
                }
            }
            Message::GoUp => {
                if let Some(parent) = self.current_path.parent() {
                    let path = parent.to_path_buf();
                    let (entries, error) = read_directory(&path);
                    self.current_path = path;
                    self.entries = entries;
                    self.error_message = error;
                }
            }
            Message::OpenFile(path) => {
                if let Err(e) = open::that(&path) {
                    self.error_message = Some(format!("Failed to open file: {}", e));
                }
            }
            Message::Refresh => {
                let (entries, error) = read_directory(&self.current_path);
                self.entries = entries;
                self.error_message = error;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            button("⬆ Up").on_press(Message::GoUp),
            button("🔄 Refresh").on_press(Message::Refresh),
        ]
        .spacing(10);

        let controls_bar = row![
            controls
        ]
        .padding(10);

        let path_bar = container(text(self.current_path.to_string_lossy()).size(14).color(iced::Color::from_rgb(0.7, 0.7, 0.7)))
            .padding(10);

        let error_banner: Element<Message> = if let Some(err) = &self.error_message {
            container(text(err).color(iced::Color::from_rgb(1.0, 0.0, 0.0)))
                .padding(10)
                .into()
        } else {
            column![].into()
        };

        let content: Element<Message> = if self.entries.is_empty() {
            container(text("Directory is empty").size(20))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            let list: Column<Message> = self.entries.iter().fold(Column::new().spacing(5), |col, entry| {
                let icon = if entry.is_dir { "📁" } else { "📄" };
                let name = text(&entry.name).width(Length::FillPortion(3));
                let size = text(&entry.size).width(Length::FillPortion(1));
                let modified = text(&entry.modified).width(Length::FillPortion(2));

                let row_content = row![
                    text(icon).width(30),
                    name,
                    size,
                    modified
                ]
                .spacing(10)
                .align_y(iced::Alignment::Center);

                let btn = button(row_content)
                    .width(Length::Fill)
                    .style(button::text)
                    .on_press(if entry.is_dir {
                        Message::NavigateTo(entry.path.clone())
                    } else {
                        Message::OpenFile(entry.path.clone())
                    });

                col.push(btn)
            });
            
            scrollable(container(list).padding(10))
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        };

        let footer = container(
            text("Created by DONGFANG WANGDAREN | 东方 旺大人")
                .size(12)
                .color(iced::Color::from_rgb(0.5, 0.5, 0.5)),
        )
        .width(Length::Fill)
        .padding(5)
        .center_x(Length::Fill);

        column![
            controls_bar,
            path_bar,
            error_banner,
            content,
            footer
        ]
        .into()
    }
}

fn read_directory(path: &Path) -> (Vec<FileEntry>, Option<String>) {
    let mut entries = Vec::new();
    let mut error_message = None;

    match fs::read_dir(path) {
        Ok(read_dir) => {
            for entry_result in read_dir {
                if let Ok(entry) = entry_result {
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();
                    let metadata = entry.metadata().ok();
                    
                    let is_dir = path.is_dir();
                    let size = if is_dir {
                        "-".to_string()
                    } else {
                        metadata.as_ref().map(|m| format_size(m.len())).unwrap_or_default()
                    };

                    let modified = metadata
                        .and_then(|m| m.modified().ok())
                        .map(format_time)
                        .unwrap_or_default();

                    entries.push(FileEntry {
                        name,
                        path,
                        is_dir,
                        size,
                        modified,
                    });
                }
            }
            // Sort: Directories first, then files. Alphabetical within groups.
            entries.sort_by(|a, b| {
                if a.is_dir == b.is_dir {
                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                } else if a.is_dir {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
        }
        Err(e) => {
            error_message = Some(format!("Error reading directory: {}", e));
        }
    }

    (entries, error_message)
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn format_time(time: SystemTime) -> String {
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
