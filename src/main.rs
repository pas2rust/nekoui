use nekoui::nyan::prelude::*;

#[component]
fn Buttons() -> impl IntoView {
    let theme = use_theme_context();
    let click = move |_| {
        theme.set(match theme.get() {
            Theme::Light => Theme::Monokai,
            Theme::Monokai => Theme::Dracula,
            Theme::Dracula => Theme::Carbon,
            Theme::Carbon => Theme::Tokyo,
            Theme::Tokyo => Theme::Dark,
            Theme::Dark => Theme::Neko,
            Theme::Neko => Theme::Rust,
            Theme::Rust => Theme::Light,
            _ => Theme::Light,
        })
    };
    let theme = move || theme.get().to_str().to_uppercase();
    view! {
       <Button
            on:click=click
        >
            {theme}
        </Button>
        <Button
            class=ButtonStyle::neko_fill().apply(ButtonStyle::circle_xs())
            on:click=click
        >
            <Icon src=Dir::PublicノAssetsノLucideノHeartCrack・svg/>
        </Button>
        <Button
            class=ButtonStyle::neko_fill().apply(ButtonStyle::circle_sm())
            on:click=click
        >
            <Icon src=Dir::PublicノAssetsノLucideノHeartCrack・svg/>
        </Button>
        <Button
            class=ButtonStyle::neko_fill().apply(ButtonStyle::lg())
            on:click=click>
            {theme}
        </Button>
         <Button
            class=ButtonStyle::neko_outline()
            on:click=click>
            {theme}
        </Button>
         <Button
            class=ButtonStyle::neko_gradient()
            on:click=click>
            {theme}
        </Button>
        <ToggleContainer>
            <ToggleButton>
                 <ToggleThumb/>
            </ToggleButton>
        </ToggleContainer>
        <ToggleContainer>
            <ToggleButton
                 class_unchecked=ToggleButtonStyle::unchecked().apply(ToggleButtonStyle::md_unchecked())
                 class_checked=ToggleButtonStyle::checked().apply(ToggleButtonStyle::md_checked())
            >
                 <ToggleThumb
                    class_unchecked=Class::new().light(ToggleThumbStyle::md_unchecked())
                    class_checked=Class::new().light(ToggleThumbStyle::md_checked())
                />
            </ToggleButton>
        </ToggleContainer>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex">
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
        </div>
        <div class="flex">
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::offline().apply(AvatarStyle::sm()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::offline().apply(AvatarDotStyle::sm()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::online().apply(AvatarStyle::sm()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::online().apply(AvatarDotStyle::sm()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::busy().apply(AvatarStyle::sm()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::busy().apply(AvatarDotStyle::sm()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::absent().apply(AvatarStyle::sm()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::absent().apply(AvatarDotStyle::sm()) />
            </AvatarContainer>
        </div>
        <div class="flex">
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::offline().apply(AvatarStyle::md()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::offline().apply(AvatarDotStyle::md()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::online().apply(AvatarStyle::md()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::online().apply(AvatarDotStyle::md()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::busy().apply(AvatarStyle::md()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::busy().apply(AvatarDotStyle::md()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::absent().apply(AvatarStyle::md()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::absent().apply(AvatarDotStyle::md()) />
            </AvatarContainer>
        </div>
         <div class="flex">
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::offline().apply(AvatarStyle::lg()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::offline().apply(AvatarDotStyle::lg()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::online().apply(AvatarStyle::lg()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::online().apply(AvatarDotStyle::lg()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::busy().apply(AvatarStyle::lg()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::busy().apply(AvatarDotStyle::lg()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::absent().apply(AvatarStyle::lg()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::absent().apply(AvatarDotStyle::lg()) />
            </AvatarContainer>
        </div>
         <div class="flex">
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::offline().apply(AvatarStyle::xl()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::offline().apply(AvatarDotStyle::xl()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::online().apply(AvatarStyle::xl()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::online().apply(AvatarDotStyle::xl()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::busy().apply(AvatarStyle::xl()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::busy().apply(AvatarDotStyle::xl()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::absent().apply(AvatarStyle::xl()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::absent().apply(AvatarDotStyle::xl()) />
            </AvatarContainer>
        </div>
         <div class="flex">
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::offline().apply(AvatarStyle::xxl()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::offline().apply(AvatarDotStyle::xxl()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::online().apply(AvatarStyle::xxl()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::online().apply(AvatarDotStyle::xxl()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::busy().apply(AvatarStyle::xxl()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::busy().apply(AvatarDotStyle::xxl()) />
            </AvatarContainer>
            <AvatarContainer>
                <AvatarImage class=AvatarStyle::absent().apply(AvatarStyle::xxl()) src=Dir::PublicノAssetsノPas・jpg />
                <AvatarDot class=AvatarDotStyle::absent().apply(AvatarDotStyle::xxl()) />
            </AvatarContainer>
        </div>
    }
}

#[component]
fn Form() -> impl IntoView {
    view! {
        <FormContainer>
            <FormInputText name="username" placeholder="Type you user here!"/>
            <FormInputText
                name="email"
                placeholder="Type your email here!"
                pattern=r"^[\w\.-]+@[\w\.-]+\.\w{3,}$"
            />
        </FormContainer>
    }
}

fn main() {
    use_mount_to_body(|| {
        view! {
            <NekoProvider>
                <LoadContainer>
                    <Loading>
                        Loading...
                    </Loading>
                    <Loaded>
                        Loaded!
                    </Loaded>
                </LoadContainer>
                <Form/>
                <App/>
                <Buttons/>
                <ChartArea/>
            </NekoProvider>
        }
    });
}
