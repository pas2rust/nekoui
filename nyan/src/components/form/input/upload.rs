use crate::components::prelude::*;
use leptos::{html, prelude::*};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Kind {
    #[default]
    All,
    PDF,
    DOC,
    DOCX,
    XLS,
    XLSX,
    PPT,
    PPTX,
    TXT,
    CSV,
    JPG,
    JPEG,
    PNG,
    APNG,
    WEBP,
    GIF,
    BMP,
    TIFF,
    SVG,
    MP3,
    WAV,
    OGG,
    FLAC,
    AAC,
    M4A,
    MP4,
    WEBM,
    MKV,
    AVI,
    MOV,
    WMV,
}

impl Kind {
    pub fn from_mime(mime: &str) -> Self {
        match mime {
            "application/pdf" => Kind::PDF,
            "application/msword" => Kind::DOC,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => Kind::DOCX,
            "application/vnd.ms-excel" => Kind::XLS,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => Kind::XLSX,
            "application/vnd.ms-powerpoint" => Kind::PPT,
            "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
                Kind::PPTX
            }
            "text/plain" => Kind::TXT,
            "text/csv" => Kind::CSV,
            "image/jpeg" => Kind::JPG,
            "image/png" => Kind::PNG,
            "image/apng" => Kind::APNG,
            "image/webp" => Kind::WEBP,
            "image/gif" => Kind::GIF,
            "image/bmp" => Kind::BMP,
            "image/tiff" => Kind::TIFF,
            "image/svg+xml" => Kind::SVG,
            "audio/mpeg" => Kind::MP3,
            "audio/wav" => Kind::WAV,
            "audio/ogg" => Kind::OGG,
            "audio/flac" => Kind::FLAC,
            "audio/aac" => Kind::AAC,
            "audio/mp4" => Kind::M4A,
            "video/mp4" => Kind::MP4,
            "video/webm" => Kind::WEBM,
            "video/x-matroska" => Kind::MKV,
            "video/x-msvideo" => Kind::AVI,
            "video/quicktime" => Kind::MOV,
            "video/x-ms-wmv" => Kind::WMV,
            _ => Kind::All,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Kind::All => "*/*",
            Kind::PDF => "application/pdf",
            Kind::DOC => "application/msword",
            Kind::DOCX => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            Kind::XLS => "application/vnd.ms-excel",
            Kind::XLSX => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            Kind::PPT => "application/vnd.ms-powerpoint",
            Kind::PPTX => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            }
            Kind::TXT => "text/plain",
            Kind::CSV => "text/csv",
            Kind::JPG | Kind::JPEG => "image/jpeg",
            Kind::PNG => "image/png",
            Kind::APNG => "image/apng",
            Kind::WEBP => "image/webp",
            Kind::GIF => "image/gif",
            Kind::BMP => "image/bmp",
            Kind::TIFF => "image/tiff",
            Kind::SVG => "image/svg+xml",
            Kind::MP3 => "audio/mpeg",
            Kind::WAV => "audio/wav",
            Kind::OGG => "audio/ogg",
            Kind::FLAC => "audio/flac",
            Kind::AAC => "audio/aac",
            Kind::M4A => "audio/mp4",
            Kind::MP4 => "video/mp4",
            Kind::WEBM => "video/webm",
            Kind::MKV => "video/x-matroska",
            Kind::AVI => "video/x-msvideo",
            Kind::MOV => "video/quicktime",
            Kind::WMV => "video/x-ms-wmv",
        }
    }
}

#[derive(Clone, Debug)]
struct FileInfo {
    name: String,
    size: f64,
    url: Option<String>,
    kind: Kind,
}

#[component]
pub fn FormInputUpload(
    #[prop(optional, default = FormInputUploadStyle::class())] class: Class,
    children: Children,
) -> impl IntoView {
    use_provide_ctx::<RwSignal<Vec<FileInfo>>>(use_rw_signal(Vec::new()));
    view! {
        <div class=class.create()>
            {children()}
        </div>
    }
}

#[component]
pub fn FormInputUploadList(
    #[prop(optional, default =  FormInputUploadStyle::class())] class: Class,
    #[prop(optional, default = false)] preview: bool
) -> impl IntoView {
    let files = use_ctx::<RwSignal<Vec<FileInfo>>>().expect("Files must be provided");
    view! {
        <ul class=class.create()>
            <For
                each=move || files.get()
                key=|f| f.url.clone()
                let:f
            >
               <li>{f.name.clone()}</li>
               <li>{f.size}</li>
               {if preview { 
                    match f.kind {
                    Kind::JPG | Kind::JPEG | Kind::PNG | Kind::APNG | Kind::WEBP
                    | Kind::GIF | Kind::BMP | Kind::TIFF | Kind::SVG => view! {
                        <img
                            class="w-1/2 h-1/2"
                            src=f.url.unwrap_or_default()
                            alt=f.name
                        />
                    }.into_any(),
                    Kind::MP4 | Kind::WEBM | Kind::OGG | Kind::MKV | Kind::AVI => view! {
                        <video controls style="max-width: 300px; max-height: 200px;">
                            <source
                                src=f.url.unwrap_or_default()
                                type=f.kind.to_str()
                            />
                            {"Your browser does not support the video tag."}
                        </video>
                    }.into_any(),
                    _ => view! {}.into_any()
               }} else {
                    view!{}.into_any()
               }
            }
             </For>
        </ul>
    }
}

#[component]
pub fn FormInputUploadTrigger(
    #[prop(optional, default = true)] multiple: bool,
    #[prop(optional, default = Kind::All)] kind: Kind,
    children: Children,
) -> impl IntoView {
    let files = use_ctx::<RwSignal<Vec<FileInfo>>>().expect("Files must be provided");
    let valid = use_input_valid_ctx();
    let input_ref = use_node_ref::<html::Input>();

    let on_change = move |ev: web_sys::Event| {
        let input: web_sys::HtmlInputElement = event_target(&ev);
        if let Some(file_list) = input.files() {
            let mut vec = Vec::new();

            for i in 0..file_list.length() {
                if let Some(file) = file_list.item(i) {
                    if kind != Kind::All && file.type_() != kind.to_str() {
                        valid.set(Valid::Error);
                    } else {
                        valid.set(Valid::Success);
                    }

                    let url = web_sys::Url::create_object_url_with_blob(&file).ok();
                    let mime = file.type_();
                    let detected_kind = Kind::from_mime(&mime);
                    vec.push(FileInfo {
                        name: file.name(),
                        size: file.size(),
                        url,
                        kind: detected_kind,
                    });
                }
            }

            files.update(|current| {
                current.extend(vec);
            });
        }
    };

    view! {
        <label on:click=move |_| {
            input_ref.get().unwrap().click();
        }>
            {children()}
        </label>
        <input
            node_ref=input_ref
            type="file"
            multiple=multiple
            accept=kind.to_str()
            on:change=on_change
            class="hidden"
        />
    }
}
