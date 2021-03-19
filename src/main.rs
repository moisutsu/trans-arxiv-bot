use anyhow::Result;
use trans_arxiv_bot::TwitterClient;

#[tokio::main]
async fn main() -> Result<()> {
    let twitter_client = TwitterClient::new().await?;
    let long_text = "Rust言語は速度、並行性、安全性を言語仕様として保証するC言語、C++に代わるシステムプログラミング（英語版）に適したプログラミング言語を目指している[5]。2006年の開発初期は、Mozillaの従業員のグレイドン・ホアレ（Graydon Hoare）[6]の個人プロジェクトだったが、2009年にMozillaが開発に関わり始めてMozilla Researchの公式プロジェクトとなった[3]。2015年に1.0版がリリースされるまでにいくつもの破壊的な仕様変更があったが、1.0版以降は基本的には後方互換を保って6週間間隔で定期的にリリースされている。プロジェクトはオープンソースのコミュニティベース開発で進行しており[7]、言語仕様（検討段階含む）、ソースコード、ドキュメントはオープンソースライセンスで公開されている[8]。";
    twitter_client.tweet_long_text(long_text).await?;
    Ok(())
}
