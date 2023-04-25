import { Messages } from "./enUS";

const messages: Messages = {
  lang: "Русский",

  "common.cancel": "Отмена",
  "common.ok": "Хорошо",
  "common.copy": "копировать",
  "common.delete": "удалить",
  "common.copy.success": "Скопировано в буфер обмена",
  "common.network.error.timeout": "Время ожидания сети",
  "common.network.error.connect": "Ошибка подключения к сети",
  "common.newVersion": "Новая версия",

  "window.setting": "Настройки",

  "chat.new": "Новый чат",
  "chat.casual.title": "Повседневный чат",
  "chat.new.defaultTitle": "Новый чат",
  "chat.conversations": "Разговоры",
  "chat.inputNameHint": "Пожалуйста, введите название чата",
  "chat.delivered": "доставлено",
  "chat.exportMarkdown": "Экспортировать",
  "chat.rename": "Переименовать",
  "chat.stick": "Закрепить",
  "chat.unstick": "Открепить",
  "chat.archive": "Архив",
  "chat.busy": "Пожалуйста, дождитесь завершения предыдущего ответа.",

  "chat.explorer.hidePinned": "Скрыть закрепленное",
  "chat.explorer.showPinned": "Показать закрепленные",
  "chat.explorer.hideArchived": "Скрыть архив",
  "chat.explorer.showArchived": "Показать в архиве",

  "chat.message.resend": "повторить",
  "chat.message.delete": "удалить",
  "chat.message.delete.hint": "Вы уверены, что хотите удалить это сообщение?",
  "chat.message.stopReply": "Перестать отвечать",

  "chat.prompt.changed": "Подсказка изменена на: {name}",

  "chat.config.model": "Модель",
  "chat.config.model.hint": "Идентификатор используемой модели.",
  "chat.config.maxBacktrack": "Максимальный возврат",
  "chat.config.maxBacktrack.hint":
    "Максимальное количество возвратов, 0 означает отсутствие ограничений",
  "chat.config.temperature": "Температура",
  "chat.config.temperature.hint":
    "Какую температуру выборки использовать, от 0 до 2. Более высокие значения, такие как 0,8, сделают вывод более случайным, а более низкие значения, такие как 0,2, сделают его более сфокусированным и детерминированным.",
  "chat.config.topP": "Лучшее P",
  "chat.config.topP.hint":
    "Альтернатива выборке с температурой, называемая выборкой ядра, где модель учитывает результаты токенов с вероятностной массой top_p. Таким образом, 0,1 означает, что только токены, составляющие 10% наиболее вероятной массы обдуманный.",
  "chat.config.n": "Н",
  "chat.config.n.hint":
    "Сколько вариантов завершения чата генерировать для каждого входного сообщения.",
  "chat.config.stop": "Стоп",
  "chat.config.stop.hint":
    "До 4 последовательностей, в которых API перестанет генерировать новые токены.",
  "chat.config.maxTokens": "Макс. токены",
  "chat.config.maxTokens.hint":
    "Максимальное количество токенов, генерируемых при завершении чата.",
  "chat.config.presencePenalty": "Штраф за присутствие",
  "chat.config.presencePenalty.hint":
    "Число от -2.0 до 2.0. Положительные значения штрафуют новые токены в зависимости от того, появляются ли они в тексте до сих пор, увеличивая вероятность того, что модель будет говорить о новых темах.",
  "chat.config.frequencyPenalty": "Штраф за частоту",
  "chat.config.frequencyPenalty.hint":
    "Число от -2.0 до 2.0. Положительные значения штрафуют новые токены в зависимости от их текущей частоты в тексте, уменьшая вероятность того, что модель дословно повторит одну и ту же строку.",

  "chat.export": "Экспорт",

  "prompt.new": "Новая подсказка",
  "prompt.prompts": "Подсказки",
  "prompt.inputNameHint": "Введите название подсказки",
  "prompt.newChat": "Новый чат",
  "prompt.rename": "Переименовать",
  "prompt.update.success": "Приглашение успешно обновлено",

  "prompt.market.prompts": "Рынок подсказок",
  "prompt.market.actions.install": "Установить",
  "prompt.market.actions.newChat": "Новый чат",
  "prompt.market.install.success": "Запрос успешно установлен",

  "plugin.market.plugins": "Рынок плагинов",
  "plugin.market.actions.install": "Установить",
  "plugin.market.actions.update": "Обновить",
  "plugin.market.actions.uninstall": "Удалить",

  "config.setting": "Настройка",

  "setting.upgrade.newVersion": "Доступна новая версия",
  "setting.upgrade.cancel": "Позже",
  "setting.upgrade.upgrade": "Обновить",
  "setting.upgrade.downloading": "Загрузка...",
  "setting.upgrade.relaunch": "Перезапустить",
  "setting.upgrade.later": "Позже",
  "setting.upgrade.download.success": "Загрузка выполнена успешно",
  "setting.upgrade.restart.hint":
    "Пожалуйста, перезапустите приложение, чтобы применить обновление.",

  "setting.locale": "Язык",
  "setting.apiKey": "Ключ API",
  "setting.proxy": "Прокси",
  "setting.theme": "Тема",
  "setting.theme.system": "Система",
  "setting.theme.dark": "Темный",
  "setting.theme.light": "Светлый",
  "setting.forwardUrl": "URL-адрес пересылки",
  "setting.forwardApiKey": "Переадресовать ключ API",
  "setting.port": "Порт",
  "setting.webPage": "Веб-страница",
  "setting.enableWebServer": "Включить веб-сервер",
  "setting.hideTaskbar": "Скрыть панель задач",
  "setting.hideMainWindow": "Скрыть главное окно",
  "setting.needRestart.hint":
    "Следующие настройки вступят в силу после перезапуска приложения",
};

export default messages;
