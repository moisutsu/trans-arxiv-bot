<h1 align="center">Welcome to trans-arxiv-bot 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000" />
  <a href="https://github.com/moisutsu/trans-arxiv-bot/blob/main/LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
  <a href="https://twitter.com/moisutsu" target="_blank">
    <img alt="Twitter: moisutsu" src="https://img.shields.io/twitter/follow/moisutsu.svg?style=social" />
  </a>
</p>

A Twitter bot that tweets translated arXiv paper summaries.

It's actually working on [this Twitter account](https://twitter.com/trans_arxiv_bot).

## Install

```sh
cargo install trans-arxiv-bot
```

## Usage

By default, it fetches the papers published in the last hour from arXiv, translates them from English to Japanese, and tweets them. The default category is `cs.CL`.

```sh
trans-arxiv-bot
```

Several options can be specified in the command line arguments, as follows.

- category (`--category`)
- source language (`--source-lang`) and target language (`--target-lang`)
- How many days or hours of the most recent paper to fetch. days (`--range-days`), hours (`--range-hours`)

You can also check this from `trans-arxiv-bot --help`.

## Author

👤 **moisutsu**

* Twitter: [@moisutsu](https://twitter.com/moisutsu)
* Github: [@moisutsu](https://github.com/moisutsu)

## Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2021 [moisutsu](https://github.com/moisutsu).<br />
This project is [MIT](https://github.com/moisutsu/trans-arxiv-bot/blob/main/LICENSE) licensed.

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
