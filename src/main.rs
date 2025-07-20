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
            class=ButtonStyle::neko_fill().apply(ButtonStyle::lg())
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
         <Button
            class=ButtonStyle::neko_gradient()
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
        <AvatarContainer>
            <AvatarImage class=AvatarStyle::online().apply(AvatarStyle::xs()) src=Dir::PublicノAssetsノPas・jpg />
            <AvatarDot class=AvatarDotStyle::online().apply(AvatarDotStyle::xs()) />
        </AvatarContainer>
        <AvatarContainer>
            <AvatarImage class=AvatarStyle::busy().apply(AvatarStyle::xs()) src=Dir::PublicノAssetsノPas・jpg />
            <AvatarDot class=AvatarDotStyle::busy().apply(AvatarDotStyle::xs()) />
        </AvatarContainer>
        <AvatarContainer>
            <AvatarImage class=AvatarStyle::absent().apply(AvatarStyle::xs()) src=Dir::PublicノAssetsノPas・jpg />
            <AvatarDot class=AvatarDotStyle::absent().apply(AvatarDotStyle::xs()) />
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
            <NekoProvider>
                <App/>
                <Buttons/>
                <ChartArea/>
            </NekoProvider>
        }
    });
}
