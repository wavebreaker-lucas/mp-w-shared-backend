pub mod element;
pub mod window;
pub mod utils;

pub const VK_LBUTTON: i32 = 0x01;
pub const VK_RETURN: i32 = 0x0D;

pub const CONTROL_TYPES: &[(i32, &str)] = &[
    (50000, "Button"),
    (50001, "Calendar"),
    (50002, "CheckBox"),
    (50003, "ComboBox"),
    (50004, "Edit"),
    (50005, "Hyperlink"),
    (50006, "Image"),
    (50007, "ListItem"),
    (50008, "List"),
    (50009, "Menu"),
    (50010, "MenuBar"),
    (50011, "MenuItem"),
    (50012, "ProgressBar"),
    (50013, "RadioButton"),
    (50014, "ScrollBar"),
    (50015, "Slider"),
    (50016, "Spinner"),
    (50017, "StatusBar"),
    (50018, "Tab"),
    (50019, "TabItem"),
    (50020, "Text"),
    (50021, "ToolBar"),
    (50022, "ToolTip"),
    (50023, "Tree"),
    (50024, "TreeItem"),
    (50025, "Custom"),
    (50026, "Group"),
    (50027, "Thumb"),
    (50028, "DataGrid"),
    (50029, "DataItem"),
    (50030, "Document"),
    (50031, "SplitButton"),
    (50032, "Window"),
    (50033, "Pane"),
    (50034, "Header"),
    (50035, "HeaderItem"),
    (50036, "Table"),
    (50037, "TitleBar"),
    (50038, "Separator"),
];