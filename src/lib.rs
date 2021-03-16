#![recursion_limit="640"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{ConsoleService, reader::{File, FileData, ReaderService, ReaderTask}};
use yew::events::DragEvent;

struct Model {
    link: ComponentLink<Self>,
    files: Vec<String>,
    tasks: Vec<ReaderTask>,
    reader: ReaderService,
}

enum Msg {
    AllowDrop(DragEvent),
    OnDrop(DragEvent),
    Loaded(FileData),
    Files(Vec<File>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            files: vec![],
            tasks: vec![],
            reader: ReaderService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AllowDrop(e) => e.prevent_default(),
            Msg::OnDrop(e) => {
                e.prevent_default();
                // let mut files = e.data_transfer().unwrap().files();
                // let mut result = Vec::new();
                // if let ChangeData::Files(files)
                // self.files.push(target);
            }
            Msg::Loaded(file) => {
                let info = format!("file: {:?}", file);
                self.files.push(info);
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let callback = self.link.callback(Msg::Loaded);
                    let task = self.reader.read_file(file, callback).unwrap();
                    self.tasks.push(task);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="main-container">
                <div
                    class="upload-container"
                    ondragover=self.link.callback(Msg::AllowDrop)
                    ondrop=self.link.callback(Msg::OnDrop)
                >
                    <span class="upload-icon fas fa-cloud-upload-alt"></span>
                    <div class="upload-items-container">
                        <span class="upload-message">{"Drag and Drop file"}</span>
                        <span class="upload-message">{"or"}</span>
                        <div class="upload-button">
                            <span>{"Browse"}</span>
                            <input type="file" multiple=true class="custom-file-input" onchange=self.link.callback(|value| {
                                let mut result = Vec::new();
                                ConsoleService::log(format!("{:?}", value).as_str());
                                if let ChangeData::Files(files) = value {
                                    let files = js_sys::try_iter(&files)
                                        .unwrap()
                                        .unwrap()
                                        .map(|v| File::from(v.unwrap()));
                                    result.extend(files);
                                }
                                Msg::Files(result)
                            }) />
                        </div>
                    </div>
                </div>
                <div class="uploaded-files-container">
                    <ul>{ for self.files.iter().map(|file| self.render_file(file)) }</ul>
                </div>
            </div>
        }
    }
}

impl Model {
    fn render_file(&self, file: &str) -> Html {
        html! {
            <li>{ file }</li>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
