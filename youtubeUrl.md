# youtubeのurl

## 基本

urlは2種類存在

- `https://www.youtube.com/`
- `https://youtu.be/`

以後、長いほうを`full`、短いほうを`mini`と呼ぶ。
また、`mini`は`full`にリダイレクトされる。

## 動画id

`0-9`, `A-Z`, `a-z`, `-`, `_` の64文字で構成され11文字の固定長

{id} は文字列

### full

- `https://www.youtube.com/watch?v={id}`

### mini

- `https://youtu.be/{id}`

> [!NOTE]
> `mini`は`full`のように指定(`youtu.be/watch?v={id}`)してもリダイレクトされるが、逆はできない

## 再生リスト

使用される文字列は動画idと同等で、34文字の固定長。
PLなどのprefixが割り当てられる。(おそらく20個ぐらいある)

{pl} は文字列

### 再生リスト自体のurlのみ

#### full

- `https://www.youtube.com/playlist?list={pl}`

#### mini

おそらく存在しない

### 再生リスト内の動画のurl

ほぼ意味のないオプションがある

再生リストの動画を一つ視聴し、次の動画に遷移するとurlの後に`&index={uint}`が付与される。
どんな値(文字列や数値)でも動画の視聴には全く関係ない

また、idを指定しない(空文字列)とyoutube公式の[謎の限定公開動画](https://www.youtube.com/watch?v=9xp1XWmJ_Wo)に飛ばされる

#### full

- `https://www.youtube.com/watch?v={id}&list={pl}`

#### mini

- `https://youtu.be/{id}?list={pl}`

## チャンネルID

`https://support.google.com/youtube/answer/11585688`
