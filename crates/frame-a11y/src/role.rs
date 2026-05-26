#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessibilityRole {
    None,
    Button,
    Link,
    Heading,
    Text,
    TextField,
    Checkbox,
    Toggle,
    Slider,
    ProgressIndicator,
    Image,
    List,
    ListItem,
    Grid,
    GridCell,
    Tab,
    TabList,
    TabPanel,
    Menu,
    MenuItem,
    Dialog,
    Alert,
    StatusBar,
    Toolbar,
    Navigation,
    Main,
    Header,
    Footer,
    ScrollArea,
    SliderThumb,
}

impl AccessibilityRole {
    pub fn is_interactive(&self) -> bool {
        matches!(self,
            AccessibilityRole::Button
            | AccessibilityRole::Link
            | AccessibilityRole::TextField
            | AccessibilityRole::Checkbox
            | AccessibilityRole::Toggle
            | AccessibilityRole::Slider
            | AccessibilityRole::Tab
            | AccessibilityRole::MenuItem
            | AccessibilityRole::SliderThumb
        )
    }

    pub fn is_container(&self) -> bool {
        matches!(self,
            AccessibilityRole::List
            | AccessibilityRole::Grid
            | AccessibilityRole::TabList
            | AccessibilityRole::TabPanel
            | AccessibilityRole::Menu
            | AccessibilityRole::Dialog
            | AccessibilityRole::Toolbar
            | AccessibilityRole::Navigation
            | AccessibilityRole::Main
            | AccessibilityRole::Header
            | AccessibilityRole::Footer
            | AccessibilityRole::ScrollArea
        )
    }

    pub fn is_landmark(&self) -> bool {
        matches!(self,
            AccessibilityRole::Navigation
            | AccessibilityRole::Main
            | AccessibilityRole::Header
            | AccessibilityRole::Footer
        )
    }
}
