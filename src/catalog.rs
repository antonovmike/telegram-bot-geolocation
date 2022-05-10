//use std::fmt;

pub struct CoffeeHouse {
    pub description: String,
    pub photo: String,
    pub google_maps: String,
    pub location_x: f32,
    pub location_y: f32,
}

pub fn kofe_list() -> [CoffeeHouse; 30] {
        let kmk: [CoffeeHouse; 30] = [CoffeeHouse {
			//format!("Stamba cafe").bold()
            description: String::from("
Stamba cafe
Двор с деревянным амфитеатром, в центре которого возвышается бутафорский электрический столб, лобби, напоминающее джунгли с неоновыми вывесками, книги, люстры — каждый метр хочется немножко умереть от эстетики. Здесь берем бенедикты, сырники c маскарпоне или йогурт с матчей + альтернативный кофе.
Открыты с 8:00 до 2:00 ночи
✅ альтернативный кофе: воронка, аэропресс, фильтр 
✅ можно с ноутбуками
Адрес: Merab Kostava St. 14
Инстаграм: https://www.instagram.com/cafe.stamba"),
            photo: String::from("src/resources/stamba_cafe.jpg"),
            google_maps: String::from("https://goo.gl/maps/hCTRnHS5NJYdLkNk9"),
            location_x: 41.705732,
            location_y: 44.787975,
        },
        CoffeeHouse {
            description: String::from("
Гардения Шеварнадзе
Вообще, это сад-оранжерея. Выглядит сказочно: каменные домики, рояли в кустах, канарейки, клумбы из старой обуви и куча мелочей, которые порадуют вас и ваших подписчиков. Здесь же есть кафе со сладостями и бабушкинским компотом. 
Открыты 10:00-18:00 (понедельник выходной)
❌ альтернативный кофе: нет 
❌не рекомендуем брать ноутбук, место не с рабочей атмосферой
Адрес: Khudadovi, 38
Инстаграм: https://www.instagram.com/gardeniashevardnadze"),
            photo: String::from("src/resources/gardeniya_shevarnadze.jpg"),
            google_maps: String::from("https://goo.gl/maps/x3DWwAzxnvQ2CwmT6"),
            location_x: 41.731023,
            location_y: 44.830517,
        },
        CoffeeHouse {
            description: String::from("
Фабрика
Это бывшее здание швейной фабрики: высокие потолки, простая и стильная отделка — все намекает на конструктивистское прошлое. Здесь хостел, бесплатная зона коворкинга и хенд-мейд магазинчики, есть даже барбершоп. Но это место больше, чем набор милых заведений, это точка притяжения тбилисской молодежи, экспатов и путешественников. За кофе и конфетками (!) идите в Milk Coffee.
Большинство мест открываются в 11-12, Milk будням открывается с 9.
❌ альтернативный кофе: нет, но есть холодные кофейные напитки 
✅  можно с ноутбуками
Адрес: Ninoshvili str.8
Инстаграм: http://instagram.com/fabrika_tbilisi"),
            photo: String::from("src/resources/fabrika.jpg"),
            google_maps: String::from("https://g.page/FabrikaTbilisi?share"),
            location_x: 41.709530,
            location_y: 44.802610,
        },
        CoffeeHouse {
            description: String::from("
Erti Kava
Одна из первых спешелти кофеен в Грузии. Сейчас это уже сеть кофеен (есть 2 точки в Тбилиси и 1 в Батуми). Сделана с большой любовью к Грузии: на стенах изображен гранат, а на стаканчиках — старый Тбилиси. Цены одни из самых высоких, но кофе и завтраки — достойные.
Открыты 8:00-21:00
✅ альтернативный кофе: фильтр, воронка, бариста сет, аэропресс
✅ можно с ноутбуками, есть wi-fi
Адрес: Mitropan Laghidze St, 8
Инстаграм: http://instagram.com/ertikava_coffeeroom"),
            photo: String::from("src/resources/erti_kava.jpg"),
            google_maps: String::from("https://goo.gl/maps/k5BpnCZsRBkwENsD8"),
            location_x: 41.698988,
            location_y: 44.795367,
        },
        CoffeeHouse {
            description: String::from("
Stories
Кофейня в старом Тбилиси, где можно привычно литрами пить недорогой и вкусный фильтр-кофе. Здесь есть также вино и десерты, такое европейское местечко. Скидка 10% если вы приходите со своей кружкой. 
Открыты 9:00-20:00 (в выходные с 10)
✅ альтернативный кофе: фильтр, воронка, аэропресс
✅ можно с ноутбуками
Адрес: 9 Galaktion Tabidze St
Инстаграм: https://www.instagram.com/stories.tbilisi/?hl=en"),
            photo: String::from("src/resources/stories.jpg"),
            google_maps: String::from("https://goo.gl/maps/TZQM5TZWkEwQ37HK8"),
            location_x: 41.691689,
            location_y: 44.801089,
        },
        CoffeeHouse {
            description: String::from("
Coffee Lab
Двухэтажная кофейня в районе Сабуртало, что находится не совсем в центре, но популярен у экспатов. Это одна из немногих кофеен, которая не закупает зерна, а обжаривает сама, здесь же можно купить кофейник, керамическую воронку и другие аксессуары. Удобно, что на террасе есть розетки почти у каждого столика.
Открыты 9:00-22:00
✅ альтернативный кофе: на любой вкус, выбор огромный
✅ можно с ноутбуками, есть wi-fi
Адрес: 27 Alexander Kazbegi Ave
Инстаграм: https://www.instagram.com/coffeelabgeorgia/?hl=en"),
            photo: String::from("src/resources/coffee_lab.jpg"),
            google_maps: String::from("https://goo.gl/maps/2daWAVhDAbjVUpxVA"),
            location_x: 41.725648,
            location_y: 44.754978,
        },
        CoffeeHouse {
            description: String::from("
Luicoffee
Если вы любите авторские напитки и кофе с молоком, то вам сюда. Халва-латте, фисташковый раф, кофе с апельсиновым фрешем и тд. Здесь ОЧЕНЬ большое меню и можно купить даже борщ, что неудивительно — кафе открыла семья из Киева.
Открыты круглосуточно! 
✅ альтернативный кофе:можно заказать V60, фильтра нет
✅ можно с ноутбуками
Адрес: 23-23a Chavchavadze Ave
Инстаграм: https://www.instagram.com/luicoffee
Google maps: https://goo.gl/maps/BEvtVVwNFtSoehjG7"),
            photo: String::from("src/resources/luicoffee.jpg"),
            google_maps: String::from("https://goo.gl/maps/BEvtVVwNFtSoehjG7"),
            location_x: 41.709292,
            location_y: 44.768015,
        },
        CoffeeHouse {
            description: String::from("
Kikliko
Kikliko переводится как «кукареку». Это традиционное блюдо для завтрака, похоже на гренки с сыром. Здесь можно попробовать более 5 вариантов этого блюда. Есть уютная терраса в тени деревьев. Фильтр-кофе нет, есть только американо и производные эспрессо. 
Открыты 8 - 15 (выходные с 9 до 16)
❌ альтернативный кофе: нет
✅ можно с ноутбуками
Адрес: 28 Mtskheta St
Инстаграм: https://www.instagram.com/kikliko_"),
            photo: String::from("src/resources/kikliko.jpg"),
            google_maps: String::from("https://g.page/Kikliko?share"),
            location_x: 41.705725,
            location_y: 44.769619,
        },
        CoffeeHouse {
            description: String::from("
Lolita
Современная Грузия, туристы и стиль в каждой детали. Вкусно, модно. громко, инстаграмно. Заказ можно делать через приложение без участия официанта. Есть недорогой фильтр-кофе, но очень маленького размера:(
Открыты с 11 до 2:00 ночи 
✅ альтернативный кофе: есть только фильтр
✅ можно с ноутбуками, есть wi-fi
Адрес: 7 Tamar Chovelidze St
Инстаграм: https://www.instagram.com/lolita.tbilisi"),
            photo: String::from("src/resources/lolita.jpg"),
            google_maps: String::from("https://goo.gl/maps/QiATrYmw8xHcVaT59"),
            location_x: 41.705671,
            location_y: 44.786925,
        },
        CoffeeHouse {
            description: String::from("
Maria Magdalena
Стильное место спрятанном в зеленом дворике. Европейская еда, боулы и все такое. В лавке при ресторане можно купить растение. Как найти? Поднимитесь от м.Руставели по улице Зандукели, оттуда первый поворот направо. Пройдите сквозь кирпичную арку и двигайтесь вниз по тропинке мимо «Кето и Коте», пока не обнаружите зелёную дверь.
Открыты 11:00-23:00
❌ альтернативный кофе: нет
❌не рекомендуем брать ноутбук, место не с рабочей атмосферой
Адрес: 5 Mikheil Zandukeli Dead End
Инстаграм: https://www.instagram.com/mariamagdalina.tbilisi"),
            photo: String::from("src/resources/maria_magdalena.jpg"),
            google_maps: String::from("https://goo.gl/maps/DdRrKJpU12Q3yB7R9"),
            location_x: 41.704922,
            location_y: 44.788100,
        },
        CoffeeHouse {
            description: String::from("
Kikodze
Бар с завтраками и авторскими коктейлями в здании бывшего винного завода. Самое популярное и инстаграмное блюдо  — сырники. Пышные, воздушные, круглые со сметаной и вареньем.  Рекомендация для более голодных  —  Чижи-Бижи — грузинская версия шакшуки с очень большим количеством томатов. Есть эспрессо-меню.
Открыты с 9 до 1:00 ночи (в пятницу и субботу работают до 3 ночи)
❌ альтернативный кофе: нет
✅ можно с ноутбуками, есть wi-fi
Адрес: 1 Vasil Petriashvili Street
Инстаграм: https://www.instagram.com/kikodzebar"),
            photo: String::from("src/resources/kikodze.jpg"),
            google_maps: String::from("https://goo.gl/maps/vWikLD4v2fL8ZpV97"),
            location_x: 41.708215,
            location_y: 44.788091,
        },
        CoffeeHouse {
            description: String::from("
Shavi Coffee
Очень вкусный кофе, обжаривают зерна сами прямо при кофейне. Попробуйте ферментированные зерна Эфиопии! Приятный плейлист, домашняя выпечка  - идеальное место для начала дня. Здесь также можно купить аксессуары (например, аэропресс домой)
Режим работы с 8:00 до 21:00
✅ альтернативный кофе: да, полный ассортимент
✅ можно с ноутбуками, но мало места, есть wi-fi
Адрес: 40 Zandukeli Street
Инстаграм: https://www.instagram.com/shavi.coffee"),
            photo: String::from("src/resources/shavi_coffee.jpg"),
            google_maps: String::from("https://goo.gl/maps/UaKHDeA3zXddsv7ZA"),
            location_x: 41.704129,
            location_y: 44.784089,
        },
        CoffeeHouse {
            description: String::from("
PULP
Кофейня с берлинским вайбом и очень вкусным тирамису. Кофе наливается в керамические кружки местного бренда 1300 Ceramic Studio. 
Открыты Пн-Чт 9:30-19, Пт 9:30-20, Сб 10-20, Вс 10-18
✅ альтернативный кофе: да, полный ассортимент
✅ можно с ноутбуками, есть wi-fi
Адрес: 22 Simon Janashia
Инстаграм: https://www.instagram.com/pulp.tbilisi/?hl=en"),
            photo: String::from("src/resources/pulp.jpg"),
            google_maps: String::from("https://goo.gl/maps/LNZq645yt9CwP7WdA"),
            location_x: 41.705987,
            location_y: 44.781482,
        },
        CoffeeHouse {
            description: String::from("
They said books
Кофейня при книжном с лучшим в Тбилиси морковным тортом. Столиков немного, можно в тишине полистать книгу и выпить кофе на зернах из Coffee LAB (местный обжарщик с кофейней в Сабуртало)
Открыты с 12 до 9 вечера
✅ альтернативный кофе: есть фильтр
✅ можно с ноутбуками
Адрес: 10 Giorgi Akhvlediani St
Инстаграм: https://www.instagram.com/theysaidbooks.coffeeshop"),
            photo: String::from("src/resources/they_said_books.jpg"),
            google_maps: String::from("https://goo.gl/maps/EkA2guZ6e5L9bZpq6"),
            location_x: 41.705315,
            location_y: 44.789875,
        },
        CoffeeHouse {
            description: String::from("
Daily grind
Лучший кофе недалеко от площади Свободы. Зерна - шведский бренд Koppi и местные Shavi Coffee Roasters, готовят их с помощью красивой машины La Marzocco. Шоколадный торт и яблочный пирог - топ. Панорамные окна с видом на старый город и  красивая плитка на полу - тоже топ!
Открыты с 8 до 12 вечера
✅ альтернативный кофе: есть фильтр, аэропресс, воронка
❌места мало
Адрес: 4 Kote Afkhazi St
Инстаграм: https://www.instagram.com/dailygrindtbilisi
Google maps: https://goo.gl/maps/X4DbGjH63gpp91m19"),
            photo: String::from("src/resources/daily_grind.jpg"),
            google_maps: String::from("https://goo.gl/maps/X4DbGjH63gpp91m19"),
            location_x: 41.693138,
            location_y: 44.802658,
        },
        CoffeeHouse {
            description: String::from("
Black Cup x Valiko Bar

Режим работы с 18 до 23:00
✅ альтернативный кофе: есть воронка, аэропресс (а еще кофе с лимоном и апельсиновым фрешем)
❌ не рекомендуем брать ноутбук, место не с рабочей атмосферой
Адрес: 24 Galaktion Tabidze Street
Инстаграм: https://www.instagram.com/valiko.mansion"),
            photo: String::from("src/resources/black_cup_x_valiko_bar.jpg"),
            google_maps: String::from("https://goo.gl/maps/hx7Wvg9fSSQrwkfWA"),
            location_x: 41.689804,
            location_y: 44.801400,
        },
        CoffeeHouse {
            description: String::from("
Eleven.window
Это место находится всего в двух шагах от верхней станции фуникулера. Отличная кофейня 3-й волны, где кофе и вкусности готовятся с любовью. Плюс прекрасный магазин и место для ивентов. Гостеприимные хосты проводят ярмарки и поддерживают местных художников.
Открыты 10:00 - 21:00
✅ альтернативный кофе: есть фильтр, воронка
✅ можно с ноутбуками, есть wi-fi
Адрес: 6 Niaghvari St
Инстаграм: https://www.instagram.com/eleven.window
Google maps: https://goo.gl/maps/pxGFnwdBR1SbbKmX6"),
            photo: String::from("src/resources/eleven_window.jpg"),
            google_maps: String::from("https://goo.gl/maps/pxGFnwdBR1SbbKmX6"),
            location_x: 41.695242,
            location_y: 44.793238,
        },
        CoffeeHouse {
            description: String::from("
Nur
Спешалти микро-кофейня с приятными ценами. Можно купить зерна Neighbourhood Coffee Roasters и съесть вкусное овсяное печенье за 3 лари. Гости хвалят какао и чизкейк.
Открыты будни 10:00 - 19:00 ( выходные 11:00 - 18:00)
✅ альтернативный кофе: да, полный ассортимент
❌ кофейня очень маленькая, не расположиться с ноутом комфортно
Адрес: Lado Kavsadze 7
Инстаграм: https://www.instagram.com/nur_coffeeshop"),
            photo: String::from("src/resources/nur.jpg"),
            google_maps: String::from("https://g.page/small-local-coffee-shop?share"),
            location_x: 41.708684,
            location_y: 44.761402,
        },
        CoffeeHouse {
            description: String::from("
Lamarzocco
Небольшой кофе-бар в Ваке, кофейня от ребят, которые открыли Daily grind. Спешалти кофе, вкусный флэтуайт, несколько видов домашних веганских десертов (говорят, самое вкусное – морковный кекс). Можно попробовать вкусную местную лимончеллу. Кофейня на 4-6 посадочных мест, но перед входом есть мини-сквер со скамейками и парой пляжных кресел.
Открыты 8:30 - 18:30 (по выходным работают до 21)
✅ альтернативный кофе: да, воронка и не только
❌ кофейня маленькая, не расположиться комфортно с ноутбуком
Адрес: Ilia Chavchavadze 27
Инстаграм: https://www.instagram.com/lamarzoccoespressobar
Google maps: https://goo.gl/maps/oRNzGXPBcths3eDL6"),
            photo: String::from("src/resources/lamarzocco.jpg"),
            google_maps: String::from("https://goo.gl/maps/oRNzGXPBcths3eDL6"),
            location_x: 41.710275,
            location_y: 44.766336,
        },
        CoffeeHouse {
            description: String::from("
Jupiter
Сыры, хлеб, хумус, V-60, матча и локал варенье. Владелец очень гостеприимный. Цены на кофе выше среднего. Но все же сюда стоит заглянуть. Конечно, в наличии вино, мы в Грузии или где🕺
Открыты 8 - 23 
✅ альтернативный кофе: да, воронка и не только
✅ можно с ноутбуками, есть wi-fi
Адрес: Ivane Machabeli 4
Инстаграм: https://www.instagram.com/jupiter.tbilisi"),
            photo: String::from("src/resources/jupiter.jpg"),
            google_maps: String::from("https://goo.gl/maps/7hf4dxxduD3WupdF7"),
            location_x: 41.692065,
            location_y: 44.799947,
        },
        CoffeeHouse {
            description: String::from("
Sol • სოლ
Красочный брекфаст спот, который работает с 8 утра! Завтраки и коктейли весь день. Восхитительная шакшука с сыром, тартин с семечками, мимоза на красном апельсине и уютная терраса. 
Открыты 08:00-22:00 
❌ альтернативный кофе: нет
✅ можно с ноутбуками
Адрес: 28 Vasil Petriashvili Street
Инстаграм: https://www.instagram.com/sol.tbilisi/?hl=ru"),
            photo: String::from("src/resources/sol.jpg"),
            google_maps: String::from("https://goo.gl/maps/d6LB8nybZ3BiRUGr6"),
            location_x: 41.705638,
            location_y: 44.779638,
        },
        CoffeeHouse {
            description: String::from("
Mondo Coffee
Основатель компании Александр Беридзе пристрастился эспрессо, живя в Италии. И решил создать первый грузинский бренд, который бы импортировал итальянский кофе премиум-класса. В 2015 году была основана компания Mondo и сейчас бренд представлен в США и Грузии. Здесь огромный выбор дрип-кофе!
Открыты 09 - 20 
✅ альтернативный кофе: да
❌ мало места, не для работы с ноутбуком
Адрес: 13 a Ivane Tarkhnishvili St
Инстаграм: https://www.instagram.com/mondogeorgia"),
            photo: String::from("src/resources/mondo_coffee.jpg"),
            google_maps: String::from("https://goo.gl/maps/xG7dtLGMxHk6NxP36"),
            location_x: 41.706474,
            location_y: 44.783221,
        },
        CoffeeHouse {
            description: String::from("
Shukura
Спешалти кофейня со вкусными завтраками и небольшой барной картой. Летом особенно приятно, можно расположиться снаружи вокруг зелени и гирлянд (места мало, вместо столиков скамейки, но очень уютно). 
Режим работы с 8:00 до 23:00
✅ альтернативный кофе: да
✅ немного места, но можно расположиться с ноутбуком
Адрес: 49 Irakli Abashidze Street
Инстаграм: https://www.instagram.com/shukura.coffee/?hl=ru"),
            photo: String::from("src/resources/shukura.jpg"),
            google_maps: String::from("https://goo.gl/maps/iBtE5girFjLRjUGr8"),
            location_x: 41.708367,
            location_y: 44.760192,
        },
        CoffeeHouse {
            description: String::from("
Erti Kava
Одна из первых спешелти кофеен в Грузии. Сейчас это уже сеть кофеен (есть 2 точки в Тбилиси и 1 в Батуми). Сделана с большой любовью к Грузии: на стенах изображен гранат, а на стаканчиках — старый Тбилиси. Цены одни из самых высоких, но кофе и завтраки — достойные.
Открыты 8:00-21:00
✅ альтернативный кофе: фильтр, воронка, бариста сет, аэропресс
✅ можно с ноутбуками, есть wi-fi
Адрес: 81 Irakli Abashidze Street
Инстаграм: http://instagram.com/ertikava_coffeeroom/?hl=en"),
            photo: String::from("src/resources/erti_kava.jpg"),
            google_maps: String::from("https://goo.gl/maps/2138Kbu6YRgDfpQx9"),
            location_x: 41.708954,
            location_y: 44.755021,
        },
        CoffeeHouse {
            description: String::from("
Books from past
Книжный магазин в тбилисской квартире. Аристократическое сочетание окон в полный рост, скрипящего паркета, переходящего в бело-голубую плитку и прозрачной необъятной тюли и книг😍 Раньше здесь была полноценная кофейня с комбучей, десертами и альтернативой. 10 марта 2022 кофейня закрылась, освободив больше места для магазина, но полистать книги и выпить эспрессо по-прежнему можно. 
Открыты 12:00-20:00
✅ альтернативный кофе: фильтр 
✅ можно с ноутбуками
Адрес: 10 Giorgi Akhvlediani St, flat 2
Инстаграм: https://www.instagram.com/booksfrompast.shop"),
            photo: String::from("src/resources/books_from_past.jpg"),
            google_maps: String::from("https://goo.gl/maps/fNmSkXiijs65GNcj8"),
            location_x: 41.705389,
            location_y: 44.789874,
        },
        CoffeeHouse {
            description: String::from("
Depo
Неприметное здание с приятным пространством внутри и летней веранда на крыше (тбилисский руфтоп, не имеющий ничего общего с московским фудкортом). Регулярно что-то происходит — диджей-сеты, маркеты, благотворительные ивенты. С 10:00 до 14:00 сервируют завтраки, а обеды —  с 12:00 до 16:00 по будням (2 блюда за 20Gel, топовый томатный копчёный суп).
Открыты 10:00-00:00
Адрес: Shalva Ghambashldze, 10
Инстаграм: https://www.instagram.com/depo.tbilisi/
Google maps: https://goo.gl/maps/Wj4kqwcTjEJ9KVm36"),
            photo: String::from("src/resources/depo.jpg"),
            google_maps: String::from("https://goo.gl/maps/Wj4kqwcTjEJ9KVm36"),
            location_x: 41.708348,
            location_y: 44.784006,
        },
        CoffeeHouse {
            description: String::from("
Unity kava
ComUNITY of specialty coffee admirers — указано в инстаграме проекта. Кофейня открылась 25 апреля 2022г. Пэт и альтернатива фрэндли. Возьмите надуги чизкейк и садитесь на крошечную солнечную террасу наблюдать за песиками.
 Открыты 8-15:00 (т.к. кофейня только появилась, режим может меняться, проверяйте в инстаграм)
✅ альтернативный кофе: есть
✅ можно с ноутбуками, но места немного
Адрес: Shalva Dadiani, 20
Инстаграм: https://www.instagram.com/unity_kava/
Google maps: https://goo.gl/maps/uLTnN8h7vvWC6fkLA"),
            photo: String::from("src/resources/unity_kava.jpg"),
            google_maps: String::from("https://goo.gl/maps/uLTnN8h7vvWC6fkLA"),
            location_x: 41.691128,
            location_y: 44.802190,
        },
        CoffeeHouse {
            description: String::from("
APC
Новая (открылась в апреле 2022) французская булочная в районе Ваке.  По совместительству картинная галерея. Владелица  — профессиональный кондитер из Франции Jullian Hequet. Эклеры, тарталетки, улитки и, конечно, круассаны. Кофейная карта как в Париже: крепкий эспрессо и латте в высоких бокалах. 
Открыты ВТ-ЧТ 10:00-20:00, ПТ-ВС 11:00-21:00
❌альтернативный кофе: нет
❌не расположиться комфортно с ноутбуком
Адрес: 10 I.Abashidze 
Инстаграм: https://www.instagram.com/apc.georgia/
Google maps: https://goo.gl/maps/fpUgRZEc15Akw7GN7"),
            photo: String::from("src/resources/apc.jpg"),
            google_maps: String::from("https://goo.gl/maps/fpUgRZEc15Akw7GN7"),
            location_x: 41.706840,
            location_y: 44.772267,
        },
        CoffeeHouse {
            description: String::from("
Sablée
Булочная в самом начале Ваке. Всевозможные десерты, баночки с вареньем и соленой карамелью, конфеты, печенье, хлеб (на пасху ребята делали куличи и творожную пасху), можно заказать целый торт. Не знаете с чего начать — берите слойку со сливочно-лимонным кремом и миндалем. Есть сытная еда (сэндвичи и салаты). Espresso-based кофейное меню.
Открыты c 09:00 до 22:00
❌альтернативный кофе: нет
❌не расположиться комфортно с ноутбуком
Адрес: Kekelidze street 1
Фейсбук: https://www.facebook.com/sableebakery/
Google maps: https://goo.gl/maps/SqyQqSWqJLPBhq298"),
            photo: String::from("src/resources/sablee.jpg"),
            google_maps: String::from("https://goo.gl/maps/SqyQqSWqJLPBhq298"),
            location_x: 41.708389,
            location_y: 44.778489,
        },
        CoffeeHouse {
            description: String::from("
Groovy roasters
Спешелти-кофейня недалеко от людной Fabrika. Ходит молва про местный шоколадный веганский кекс с вишней и свёклой. Если не хотите сладкое, то берите сэндвич с карамелизированным сыром и хлебом на закваске. Большой выбор кофейного зерна (которые они сами обжаривают). Есть эспрессо-тоник, вино, пиво и даже самбука. Место новое, так что ждем расширения ассортимента меню. 
Открыты с 8 до 8
✅альтернативный кофе: V60, аэропресс, фильтр, есть матча
✅можно с ноутбуком
Адрес: Ninoshvili 17
Инстаграм: https://www.instagram.com/groovyroasters/
Google maps: https://goo.gl/maps/2CVxQkwiHB1sbnhb6"),
            photo: String::from("src/resources/groovy_roasters.jpg"),
            google_maps: String::from("https://goo.gl/maps/2CVxQkwiHB1sbnhb6"),
            location_x: 41.711521,
            location_y: 44.801714,
        },
    ];
    kmk
}
