use nyan::prelude::*;

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
            <GlobalProvider>
                <Buttons/>
                <App/>
                <Spinner/>
                <ChartArea/>
            </GlobalProvider>
        }
    });
}
