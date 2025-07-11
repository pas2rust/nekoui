use leptos::prelude::*;

#[component]
pub fn SnowFall() -> impl IntoView {
    view! {
        <div class="fixed overflow-hidden z-50 top-0 left-0 h-screen w-screen pointer-events-none">
            <div class="absolute top-[1%] left-[5%] before:content-['❄️'] before:absolute before:animate-snow-fall-loop before:text-xl before:opacity-80"></div>
            <div class="absolute top-[4%] left-[22%] before:content-['❄️'] before:absolute before:animate-snow-fall-slow-loop before:text-3xl before:opacity-60"></div>
            <div class="absolute top-[6%] left-[75%] before:content-['❄️'] before:absolute before:animate-snow-fall-fast-loop before:text-lg before:opacity-75"></div>
            <div class="absolute top-[10%] left-[13%] before:content-['❄️'] before:absolute before:animate-snow-fall-slow-loop before:text-2xl before:opacity-50"></div>
            <div class="absolute top-[15%] left-[40%] before:content-['❄️'] before:absolute before:animate-snow-fall-loop before:text-4xl before:opacity-70"></div>
            <div class="absolute top-[18%] left-[85%] before:content-['❄️'] before:absolute before:animate-snow-fall-fast-loop before:text-xs before:opacity-40"></div>
            <div class="absolute top-[22%] left-[30%] before:content-['❄️'] before:absolute before:animate-snow-fall-slow-loop before:text-lg before:opacity-90"></div>
            <div class="absolute top-[28%] left-[70%] before:content-['❄️'] before:absolute before:animate-snow-fall-loop before:text-xl before:opacity-65"></div>
            <div class="absolute top-[33%] left-[18%] before:content-['❄️'] before:absolute before:animate-snow-fall-loop before:text-2xl before:opacity-85"></div>
            <div class="absolute top-[38%] left-[64%] before:content-['❄️'] before:absolute before:animate-snow-fall-slow-loop before:text-xl before:opacity-55"></div>
            <div class="absolute top-[42%] left-[3%] before:content-['❄️'] before:absolute before:animate-snow-fall-fast-loop before:text-sm before:opacity-40"></div>
            <div class="absolute top-[48%] left-[38%] before:content-['❄️'] before:absolute before:animate-snow-fall-loop before:text-lg before:opacity-75"></div>
            <div class="absolute top-[53%] left-[92%] before:content-['❄️'] before:absolute before:animate-snow-fall-fast-loop before:text-3xl before:opacity-80"></div>
            <div class="absolute top-[58%] left-[27%] before:content-['❄️'] before:absolute before:animate-snow-fall-loop before:text-base before:opacity-50"></div>
            <div class="absolute top-[61%] left-[82%] before:content-['❄️'] before:absolute before:animate-snow-fall-slow-loop before:text-xs before:opacity-85"></div>
            <div class="absolute top-[66%] left-[35%] before:content-['❄️'] before:absolute before:animate-snow-fall-fast-loop before:text-4xl before:opacity-60"></div>
            <div class="absolute top-[72%] left-[62%] before:content-['❄️'] before:absolute before:animate-snow-fall-loop before:text-xl before:opacity-75"></div>
            <div class="absolute top-[75%] left-[15%] before:content-['❄️'] before:absolute before:animate-snow-fall-slow-loop before:text-sm before:opacity-70"></div>
            <div class="absolute top-[80%] left-[48%] before:content-['❄️'] before:absolute before:animate-snow-fall-fast-loop before:text-lg before:opacity-65"></div>
            <div class="absolute top-[85%] left-[8%] before:content-['❄️'] before:absolute before:animate-snow-fall-loop before:text-2xl before:opacity-50"></div>
            <div class="absolute top-[89%] left-[79%] before:content-['❄️'] before:absolute before:animate-snow-fall-slow-loop before:text-xs before:opacity-80"></div>
            <div class="absolute top-[92%] left-[25%] before:content-['❄️'] before:absolute before:animate-snow-fall-fast-loop before:text-xl before:opacity-60"></div>
            <div class="absolute top-[97%] left-[53%] before:content-['❄️'] before:absolute before:animate-snow-fall-loop before:text-lg before:opacity-75"></div>
            <div class="absolute top-[99%] left-[90%] before:content-['❄️'] before:absolute before:animate-snow-fall-fast-loop before:text-3xl before:opacity-80"></div>
        </div>
    }
}
