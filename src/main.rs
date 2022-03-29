use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::{BoxExt, GtkWindowExt, OrientableExt};
use relm4::{adw::{self, traits::{PreferencesRowExt, ActionRowExt}, prelude::{WidgetExt}}, gtk::{self, SelectionMode}, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets, factory::{FactoryVec, FactoryPrototype}, view};

const START_ICON : &str = r#"media-playback-start-symbolic"#;
const START_TOOLTIP : &str = r#"Start toolbox"#;

const SHUTDOWN_ICON : &str = r#"system-shutdown-symbolic"#;
const SHUTDOWN_TOOLTIP : &str = r#"Stop toolbox"#;

const UPDATE_ICON : &str = r#"software-update-available-symbolic"#;
const UPDATE_TOOLTIP : &str = r#"Update all applications inside of the toolbox"#;

const APP_ICON : &str = r#"view-grid-symbolic"#;
const APP_TOOLTIP : &str = r#"Select applications to showup in the application menu"#;

const TERMINAL_ICON : &str = r#"utilities-terminal-symbolic"#;
const TERMINAL_TOOLTIP : &str = r#"Open terminal inside of toolbox"#;

const SETTINGS_ICON : &str = r#"applications-system-symbolic"#;
const SETTINGS_TOOLTIP : &str = r#"Open toolbox settings"#;
struct AppModel {
    toolboxes: FactoryVec<ToolboxContainer>
}

enum AppMsg {
    Increment,
    Decrement,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Increment => {
            }
            AppMsg::Decrement => {
            }
        }
        true
    }
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = adw::ApplicationWindow {
            set_default_width: 800,
            set_default_height: 600,

            set_content: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,

                append = &adw::HeaderBar {
                    set_title_widget = Some(&gtk::Label) {
                        set_label: "Toolbox Tuner",
                    }
                },

                append = &gtk::ListBox {
                    set_selection_mode: SelectionMode::None,
                    set_margin_all: 30,
                    set_css_classes: &["boxed-list"],
                    factory!(model.toolboxes)
                }
                
            },
        }
    }
}

// FACTORY
enum ToolboxStatus {
    Stopped,
    Running,
}

impl Default for ToolboxStatus {
    fn default() -> Self {
        ToolboxStatus::Stopped
    }
}

#[derive(Default)]
struct ToolboxContainer{
    name: String,
    status: ToolboxStatus,
    update_available: bool,
}

#[derive(Debug)]
struct FactoryWidgets {
    action_row: adw::ActionRow,
}

impl FactoryPrototype for ToolboxContainer {
    type Factory = FactoryVec<Self>;
    type Widgets = FactoryWidgets;
    type Root = adw::ActionRow;
    type View = gtk::ListBox;
    type Msg = AppMsg;

    fn init_view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        sender: Sender<Self::Msg>,
    ) -> Self::Widgets {
           
        view!{
            suffix_box = &gtk::Box{
                append = &gtk::Button::from_icon_name(APP_ICON) {
                    set_margin_start: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_tooltip_text: Some(APP_TOOLTIP),
                    set_css_classes: &["flat"],
                },
                append = &gtk::Button::from_icon_name(TERMINAL_ICON) {
                    set_margin_start: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_tooltip_text: Some(TERMINAL_TOOLTIP),
                    set_css_classes: &["flat"],
                },
                append = &gtk::Button::from_icon_name(SETTINGS_ICON) {
                    set_margin_start: 10,set_margin_start: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_tooltip_text: Some(SETTINGS_TOOLTIP),
                    set_css_classes: &["circular"],
                },
            }
        };

        if self.update_available {
            view!{
                update_button = &gtk::Button::from_icon_name(UPDATE_ICON) {
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_margin_end: 10,
                    set_tooltip_text: Some(UPDATE_TOOLTIP),
                    set_css_classes: &["suggested-action"],
                }
            };
            suffix_box.prepend(&update_button);
        }


        let is_on = true;

        let mut status_button_tooltip = START_TOOLTIP;
        let mut status_button_icon = START_ICON;

        match &self.status {
            &ToolboxStatus::Running => {
                status_button_tooltip = SHUTDOWN_TOOLTIP;
                status_button_icon = SHUTDOWN_ICON;
            },
            &ToolboxStatus::Stopped => {
                status_button_tooltip = START_TOOLTIP;
                status_button_icon = START_ICON;
            }
        }
            
        view! {
            action_row = &adw::ActionRow {
                set_title: &self.name,
                set_subtitle: "additional information",
                add_prefix = &gtk::Box {
                    append = &gtk::Button::from_icon_name(status_button_icon) {
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_tooltip_text: Some(status_button_tooltip),
                        set_css_classes: &["circular"],
                    },
                },
                add_suffix: &suffix_box,
            }
        };
        FactoryWidgets { action_row }
    }

    fn view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        //widgets.action_row.set_label(&self.name.to_string());
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.action_row
    }

    fn position(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
    ) -> <Self::View as relm4::factory::FactoryView<Self::Root>>::Position {    }
}

// END_FACTORY

fn main() {
    let mut factory_vec = FactoryVec::new();
    factory_vec.push(ToolboxContainer{name: "fedora-toolbox-35".to_string(), status: ToolboxStatus::Running, update_available: false});
    factory_vec.push(ToolboxContainer{name:  "Latex".to_string(), status: ToolboxStatus::Running, update_available: false});
    factory_vec.push(ToolboxContainer{name: "Rust".to_string(), status: ToolboxStatus::Stopped, update_available: true});
    
    let model = AppModel {
        toolboxes: factory_vec
    };
    let app = RelmApp::new(model);
    app.run();
}
