<h1 align="center">Welcome to trans-arxiv-bot 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.1.6-blue.svg?cacheSeconds=2592000" />
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

When activated, it will automatically fetch papers at regular intervals. And when the latest paper is posted, it will translate the summary and tweet it. To avoid continuous tweets, tweets will be sent at regular intervals.

```sh
trans-arxiv-bot
```

Several options can be specified in the command line arguments, as follows.

- Category `--category` [default: `cs.CL`]
- Source language for translation `--source-lang` [default: `en`]
- Target language for translation `--target-lang` [default: `ja`]
- How many minutes to update and tweet `--update-frequency` [default: `15`]

You can also check this from `trans-arxiv-bot --help`.

You will also need to set some values in the environment variables.

- Twitter
  - API_KEY
  - API_SECRET_KEY
  - ACCESS_TOKEN
  - ACCESS_TOKEN_SECRET
- translate
  - TRANSLATE_URL (see: [Google 翻訳 API を無料で作る方法 - Qiita](https://qiita.com/satto_sann/items/be4177360a0bc3691fdf) and [google_translate_api](https://gist.github.com/moisutsu/6d5b1721d4c4e4aa7e6184f2a6f557d5))

Alternatively, you can use Docker to run it.
See [docker-compose.yml](https://github.com/moisutsu/trans-arxiv-bot/blob/main/docker-compose.yml) and [.env.sample](https://github.com/moisutsu/trans-arxiv-bot/blob/main/.env.sample) for details.

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
