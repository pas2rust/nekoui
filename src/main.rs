use nekoui::nyan::prelude::*;

#[component]
fn Buttons() -> impl IntoView {
    let theme = use_theme_context();

    view! {
       <Button
            on:click=move |_| {
                theme.set(match theme.get() {
                    Theme::Light => Theme::Monokai,
                    Theme::Monokai => Theme::Dracula,
                    Theme::Dracula => Theme::Carbon,
                    Theme::Carbon => Theme::Tokyo,
                    Theme::Tokyo => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    _ => Theme::Light,
                });
        }>
            {move || theme.get().to_str().to_uppercase()}
        </Button>
        <Button
            class=ButtonStyle::neko_fill().m(TwUnit::Twelve)
            on:click=move |_| {
                theme.set(match theme.get() {
                    Theme::Light => Theme::Monokai,
                    Theme::Monokai => Theme::Dracula,
                    Theme::Dracula => Theme::Carbon,
                    Theme::Carbon => Theme::Tokyo,
                    Theme::Tokyo => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    _ => Theme::Light,
                });
        }>
            {move || theme.get().to_str().to_uppercase()}
        </Button>
         <Button
            class=ButtonStyle::neko_outline()
            on:click=move |_| {
                theme.set(match theme.get() {
                    Theme::Light => Theme::Monokai,
                    Theme::Monokai => Theme::Dracula,
                    Theme::Dracula => Theme::Carbon,
                    Theme::Carbon => Theme::Tokyo,
                    Theme::Tokyo => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    _ => Theme::Light,
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
            <AvatarImage src=Dir::PublicノAssetsノPas・jpg />
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
            <NekoノUI・Nyan>
                <Buttons/>
                <ChartArea/>
            </NekoノUI・Nyan>
        }
    });
}
