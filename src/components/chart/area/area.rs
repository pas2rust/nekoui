use crate::components::chart::{
    bar_chart::{BarChart, BarChartOptions, BarOptions, Direction},
    grid::GridOptions,
    line_chart::{LineChart, LineChartOptions, LineOptions},
    pie_chart::{PieChart, PieChartOptions, PieOptions},
    radial_chart::{RadialChart, RadialChartOptions, RadialOptions},
    use_canvas::*,
};
use leptos::prelude::*;
use tailwind::components::prelude::*;

fn create_line_chart() -> LineChart {
    LineChart::new()
        .options(LineChartOptions {
            padding: PaddingOptions::new().vertical(10).horizontal(10).global(10),
            line: LineOptions::new()
                .width(3.0)
                .point_radius(4.0)
                .color(Color::White)
                .point_color(Color::White),
            font: FontOptions::new().text_color(Color::White),
            grid: GridOptions::new(),
        })
        .series([
            vec![
                // Série Vermelha/Original
                ChartData::new()
                    .value(75)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(30)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(10)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(60)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(30)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(10)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(60)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(30)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(10)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(400)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(300)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(120)
                    .color(Color::Red(Shade::FiveHundred))
                    .font(FontOptions::new().text_color(Color::White)),
            ],
            vec![
                // Série Verde
                ChartData::new()
                    .value(60)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(20)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(5)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(45)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(25)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(15)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(50)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(35)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(20)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(350)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(270)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(100)
                    .color(Color::Green(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
            ],
            vec![
                // Série Azul
                ChartData::new()
                    .value(50)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(40)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(25)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(20)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(15)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(10)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(20)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(40)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(70)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(250)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(200)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
                ChartData::new()
                    .value(90)
                    .color(Color::Blue(Shade::FourHundred))
                    .font(FontOptions::new().text_color(Color::White)),
            ],
        ])
}

fn create_bar_chart(dir: Direction) -> BarChart {
    BarChart::new()
        .options(
            BarChartOptions::new()
                .padding(PaddingOptions::new().global(5.0))
                .bar(BarOptions::new().border_radius(5))
                .direction(dir)
                .grid(GridOptions::new()),
        )
        .data([
            ChartData::new()
                .value(75)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .color(Color::Red(Shade::FiveHundred))
                .font(FontOptions::new().text_color(Color::White))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Red(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(30)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .font(FontOptions::new().text_color(Color::White))
                .color(Color::Green(Shade::FiveHundred))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Green(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(10)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .font(FontOptions::new().text_color(Color::White))
                .color(Color::Purple(Shade::FiveHundred))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Purple(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(60)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .color(Color::Red(Shade::FiveHundred))
                .font(FontOptions::new().text_color(Color::White))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Red(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(30)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .font(FontOptions::new().text_color(Color::White))
                .color(Color::Green(Shade::FiveHundred))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Green(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(10)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .font(FontOptions::new().text_color(Color::White))
                .color(Color::Purple(Shade::FiveHundred))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Purple(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(60)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .color(Color::Red(Shade::FiveHundred))
                .font(FontOptions::new().text_color(Color::White))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Red(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(30)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .font(FontOptions::new().text_color(Color::White))
                .color(Color::Green(Shade::FiveHundred))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Green(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(10)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .font(FontOptions::new().text_color(Color::White))
                .color(Color::Purple(Shade::FiveHundred))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Purple(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(400)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .color(Color::Red(Shade::FiveHundred))
                .font(FontOptions::new().text_color(Color::White))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Red(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(300)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .font(FontOptions::new().text_color(Color::White))
                .color(Color::Green(Shade::FiveHundred))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Green(Shade::FiveHundred))
                        .blur(10),
                ),
            ChartData::new()
                .value(120)
                .padding(PaddingOptions::new().vertical(1).horizontal(1))
                .font(FontOptions::new().text_color(Color::White))
                .color(Color::Purple(Shade::FiveHundred))
                .shadow(
                    ShadowOptions::new()
                        .color(Color::Purple(Shade::FiveHundred))
                        .blur(10),
                ),
        ])
}

fn create_pie_chart() -> PieChart {
    PieChart::new()
        .options(
            PieChartOptions::new()
                .padding(PaddingOptions::new().horizontal(10).vertical(10))
                .pie(
                    PieOptions::new()
                        .alpha(0.9)
                        .stroke_color(Color::White)
                        .line_width(2.0)
                        .is_doughnut(true)
                        .doughnut_ratio(0.6)
                        .show_percentage(true),
                )
                .font(
                    FontOptions::new()
                        .font("12px black Arial")
                        .text_color(Color::White),
                )
                .grid(GridOptions::new()),
        )
        .data(vec![
            ChartData::new()
                .value(40.0)
                .color(Color::Red(Shade::FiveHundred))
                .shadow(
                    ShadowOptions::new()
                        .blur(5)
                        .color(Color::Red(Shade::FiveHundred)),
                ),
            ChartData::new()
                .value(30.0)
                .color(Color::Green(Shade::FourHundred))
                .shadow(
                    ShadowOptions::new()
                        .blur(5)
                        .color(Color::Green(Shade::FourHundred)),
                ),
            ChartData::new()
                .value(20.0)
                .color(Color::Blue(Shade::FourHundred))
                .shadow(
                    ShadowOptions::new()
                        .blur(5)
                        .color(Color::Blue(Shade::FourHundred)),
                ),
            ChartData::new()
                .value(10.0)
                .color(Color::Purple(Shade::FourHundred))
                .shadow(
                    ShadowOptions::new()
                        .blur(5)
                        .color(Color::Purple(Shade::FourHundred)),
                ),
        ])
}

fn create_radial_chart() -> RadialChart {
    RadialChart::new()
        .options(
            RadialChartOptions::new()
                .padding(PaddingOptions::new().vertical(25).horizontal(15))
                .radial(
                    RadialOptions::new()
                        .alpha(0.5)
                        .stroke_color(Color::White)
                        .show_percentage(true)
                        .fill_area(true), // Adicionei fill_area para melhor visualização
                )
                .font(
                    FontOptions::new()
                        .font("12px Arial")
                        .text_color(Color::White),
                )
                .labels([
                    "A".to_string(),
                    "B".to_string(),
                    "C".to_string(),
                    "D".to_string(),
                    "E".to_string(),
                    "F".to_string(),
                    "G".to_string(),
                ]) // Adicione labels correspondentes aos dados
                .grid(GridOptions::new()), // Adicione grid se necessário
        )
        .data([
            ChartData::new()
                .value(50.0)
                .color(Color::Red(Shade::FiveHundred)),
            ChartData::new()
                .value(30.0)
                .color(Color::Green(Shade::FourHundred)),
            ChartData::new()
                .value(20.0)
                .color(Color::Blue(Shade::FourHundred)),
            ChartData::new()
                .value(15.0)
                .color(Color::Purple(Shade::FourHundred)),
            ChartData::new()
                .value(40.0)
                .color(Color::Orange(Shade::FourHundred)),
            ChartData::new()
                .value(35.0)
                .color(Color::Pink(Shade::FourHundred)),
            ChartData::new()
                .value(5.0)
                .color(Color::Yellow(Shade::FourHundred)),
        ])
}

#[component]
pub fn ChartArea() -> impl IntoView {
    view! {
        <div class="flex justify-center flex-wrap gap-4">
            <div class="border border-gray-300 rounded-lg shadow-md">
                <canvas node_ref=use_canvas(create_bar_chart(Direction::BottomUp)) class="w-full h-[500px]" />
            </div>
            <div class="border border-gray-300 rounded-lg shadow-md">
                <canvas node_ref=use_canvas(create_bar_chart(Direction::TopDown)) class="w-full h-[500px]" />
            </div>
            <div class="border border-gray-300 rounded-lg shadow-md">
                <canvas node_ref=use_canvas(create_bar_chart(Direction::LeftRight)) class="w-full h-[500px]" />
            </div>
            <div class="border border-gray-300 rounded-lg shadow-md">
                <canvas node_ref=use_canvas(create_bar_chart(Direction::RightLeft)) class="w-full h-[500px]" />
            </div>
            <div class="border border-gray-300 rounded-lg shadow-md">
                <canvas node_ref=use_canvas(create_line_chart()) class="w-full h-[500px]" />
            </div>
            <div class="border border-gray-300 rounded-lg shadow-md">
                <canvas node_ref=use_canvas(create_pie_chart()) class="w-full h-[500px]" />
            </div>
            <div class="border border-gray-300 rounded-lg shadow-md">
                <canvas node_ref=use_canvas(create_radial_chart()) class="w-full h-[800px]" />
            </div>
        </div>
    }
}
