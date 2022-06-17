Инструкция по стримам БАФа (путь с одним контейнером)
Ни разу не md, но кому не всё равно

Ставим и стартуем докер
sudo pacman -S docker
sudo systemctl start docker

Создаем "Dockerfile" (без расширения) в проекте на уровне Cargo.toml (Нужен для создания **ОБРАЗА**)
Наполняем его следующим:
```docker
FROM rust:1.61-bullseye

WORKDIR /usr/src/rust_app_with_db
COPY . .

RUN cargo install --path .

CMD ["rust_app_with_db"]
```
// Имя (в моем случае: rust_app_with_db) должно совпадать с name из Cargo.toml


Создаем файл .dockerignore
Добавляем в него "/target" (как лежит в .gitignore)


Билдим образ (параметр после тега - всё то же название из cargo.toml)
Либо: // не делал
sudo docker build --tag rust_app_with_db .
Либо: // делал
1.Добавляемся в группу пользователей с привилегированным доступом к докеру (по своему username в системе):
sudo usermod --append --groups docker yury
2.Перезапускаем сессию
3.Снова запускаем докер by
sudo systemctl start docker
4.Билдим образ из папки проекта (где лежит dockerfile)
docker build --tag rust_app_with_db .

В любом случае долго ждем...


Создаем контейнер на базе образа:
docker run -it --rm --name [имя будущего контейнера] [имя образа]
(в моем случае: docker run -it --rm --name rust_app_with_db_running rust_app_with_db)

К нему не постучаться через curl, т.к. не проброшен порт

Убить контейнер из другого терминала:
docker container kill [имя контейнера]
docker container kill rust_app_with_db_running


Проброс порта (-_-)? (вернее старт с указанием порта)
docker run --interactive --tty --publish 8000:8000 --rm --name [имя будущего контейнера] [имя образа]
docker run --interactive --tty --publish 8000:8000 --rm --name rust_app_with_db_running rust_app_with_db


Общаемся с контейнером через
curl http://127.0.0.1:8000/[any]

Пхд профит (но у меня не заработало)

