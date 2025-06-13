use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutXml {

    pub preload: Preload,

    pub events: Events,

    pub layout: Layout,
}

#[derive(Debug, Deserialize)]
struct Preload {

    #[serde(rename = "sound", default)]
    pub sounds: Vec<Asset>,

    #[serde(rename = "image", default)]
    pub images: Vec<Asset>,

    #[serde(rename = "text", default)]
    pub texts: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {

    #[serde(rename = "@src")]
    pub src: String,

    #[serde(rename = "$text")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Events {

    #[serde(rename = "lua_setup", default)]
    pub lua_setup: Option<String>,

    #[serde(rename = "lua_exit", default)]
    pub lua_exit: Option<String>,

    #[serde(rename = "lua", default)]
    pub lua_scripts: Vec<Lua>,
}

#[derive(Debug, Deserialize)]
pub struct Lua {

    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@src", default)]
    pub src: Option<String>,

    #[serde(rename = "$text", default)]
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Layout {

    #[serde(rename = "box_layout")]
    pub boxes: Vec<BoxLayout>,
}

#[derive(Debug, Deserialize)]
pub struct BoxLayout {

    pub button_bind: ButtonBind,

    pub position: String,

    pub pivot: String,

    pub size: String,

    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ButtonBind {

    #[serde(rename = "on_click")]
    pub on_click: OnClick,
}

#[derive(Debug, Deserialize)]
pub struct OnClick {

    #[serde(rename = "@event")]
    pub event: String,
}

impl LayoutXml {
    pub fn parse_xml(xml: String) -> Result<LayoutXml, quick_xml::DeError> {
        let wrapped = format!("<root>{}</root>", xml);

        let layout_xml: LayoutXml = from_str(&wrapped)?;
        Ok(layout_xml)
    }
}