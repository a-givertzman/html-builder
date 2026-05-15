use std::fmt::Display;

use sal_sync::collections::FxHashMap;
///
/// Alias для удобства и понимания
type Key = String;
///
/// ### Изолированный контекст перевода для генерации отчетов.
/// Хранит плоский словарь (ключ-значение) для конкретного языка,
/// обеспечивая отказоустойчивый перевод терминов.
#[derive(Debug)]
pub struct Translation {
    is_empty: bool,
    items: FxHashMap<Key, String>,
}
impl Translation {
    ///
    /// ### Создает новый контекст перевода из готового словаря.
    /// - `translations` - принимает коллекцию пар `ключ-перевод`
    pub fn new(translations: impl IntoIterator<Item = (Key, String)>) -> Self {
        Self {
            is_empty: false,
            items: FxHashMap::from_iter(translations),
        }
    }
    ///
    /// ### Создает пустой контекст перевода без словаря.
    pub fn empty() -> Self {
        Self {
            is_empty: true,
            items: FxHashMap::default(),
        }
    }
    ///
    /// ### Возвращает переведенную строку для заданного ключа.
    /// Если перевод в словаре отсутствует, возвращает ключ.
    pub fn tr<'a, T: Display>(&'a self, key: &'a T) -> LazyTranslation<'a, T> {
        LazyTranslation { ctx: self, key }
    }
}
impl Default for Translation {
    fn default() -> Self {
        Self::empty()
    }
}
///
/// ### Обертка которая откладывает перевод
pub struct LazyTranslation<'a, T: Display> {
    ctx: &'a Translation,
    key: &'a T,
}
//
impl<'a, T: Display> Display for LazyTranslation<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ctx.is_empty {
            // Если мапа пустая, пишем ключ напрямую в escaper/formatter без аллокаций
            return Display::fmt(self.key, f);
        }
        // Чтобы не аллоцировать String для поиска в мапе, мы можем временно 
        // отформатировать ключ в небольшую строку на стеке, если ключ короткий.
        // Но так как в вашей мапе ключом является String, нам нужна строка для поиска.
        // Оптимальный компромисс: форматируем ключ один раз.
        let key_str = self.key.to_string(); 
        match self.ctx.items.get(&key_str) {
            Some(translated_str) => f.write_str(translated_str),
            None => f.write_str(&key_str),
        }
    }
}