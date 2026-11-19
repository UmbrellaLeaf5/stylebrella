# Форматирование C++

[English version](../../../en/code-style/cpp/formatting.md)

Эта страница дополняет [общие правила форматирования](../formatting.md) правилами C++. Она охватывает `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp` и `.hh`. См. [линтинг C++](linting.md). Не запускайте clang-format для `CMakeLists.txt`, `*.cmake` или отдельных файлов данных и конфигурации.

## Оформление

Используйте clang-format с базовым стилем Google. Отступ продолжения — два пробела; элементы массива структур выравниваются по левому краю. Бинарные операторы, кроме присваивания, переносятся на начало следующей строки, список инициализации конструктора переносится перед двоеточием. Не переразбивайте комментарии на строки. Короткие функции и короткие `if`/`for` без скобок допустимы на одной строке, если помещаются; не навязывайте скобки и не меняйте управляющий поток. Для поиска `.clang-format` используйте фактическое имя исходного файла, включая обработку stdin. Примеры задают желаемый результат, даже если совместимость движка ещё нужно доработать.

До:

```cpp
void apply(const std::string& resolved_configuration_path, const Settings& resolved_settings, Logger& event_logger) {
    load(resolved_configuration_path,resolved_settings,event_logger);
}
```

После:

```cpp
void apply(
  const std::string& resolved_configuration_path,
  const Settings& resolved_settings,
  Logger& event_logger) {
  load(resolved_configuration_path, resolved_settings, event_logger);
}
```

## Пустые строки

- Для объявлений внутри блока пространства имён действует правило одной пустой строки. Оставляйте `else if`, `else` и `catch` на отдельной строке.
- Последовательные присваивания полям, например `signal.id = …` и `signal.freq = …`, остаются вместе с `Signal signal;` по общему правилу операций над одним объектом, даже если заполняют разные поля.

До:

```cpp
announce();
Signal signal;

signal.id = id;
signal.freq = freq;
if (ready) { send(signal); } else { retry(signal); }
```

После:

```cpp
announce();

Signal signal;
signal.id = id;
signal.freq = freq;

if (ready) { send(signal); }

else { retry(signal); }
```

## Разделители и приватные методы

Используйте `//` для общих разделителей и структурных комментариев в заголовках и исходниках. Doxygen должен примыкать к объявлению, а комментарий к полю — оставаться на той же строке, что и поле. В [линтинге C++](linting.md#mark-порядок-методов-и-документация) определено, какие методы считаются приватными.

```cpp
class Reader {
 public:
  Result read();

  // (no private methods)
};
```
