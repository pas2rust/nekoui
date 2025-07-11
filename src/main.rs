use leptos::prelude::*;
use nekoui::components::_shared::prelude::*;
use nekoui::components::adapters::*;
use nekoui::components::avatar::prelude::*;
use nekoui::components::avatar::status::Status;
use nekoui::components::avatar::styles::{AvatarDotStyle, AvatarStyle};
use nekoui::components::button::prelude::*;
use nekoui::components::chart::area::area::ChartArea;
use nekoui::components::fall::screen::SnowFall;
use nekoui::components::provider::global::GlobalProvider;
use nekoui::components::provider::tailwind::use_theme_context;
use nekoui::components::spinner::spinner::Spinner;
use nekoui::components::toggle::toggle::*;

#[component]
fn Buttons() -> impl IntoView {
    let theme = use_theme_context();

    view! {
       <Button
            on:click=move |_| {
                theme.set(match theme.get() {
                    Theme::Dark => Theme::Light,
                    Theme::Light => Theme::Dark,
                    _ => Theme::Dark
                });
        }>
            {move || theme.get().to_str().to_uppercase()}
        </Button>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <AvatarContainer>
            <AvatarImage src="assets/pas.jpg" />
            <AvatarDot />
        </AvatarContainer>
        <ToggleContainer>
            <ToggleButton>
                <ToggleThumb/>
            </ToggleButton>
        </ToggleContainer>
    }
}

fn main() {
    use_mount_to_body(|| {
        view! {
            <GlobalProvider>
                <Buttons/>
                <App/>
                <Spinner/>
                <ChartArea/>
            </GlobalProvider>
        }
    });
}
