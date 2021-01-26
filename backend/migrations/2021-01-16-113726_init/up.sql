-- 管理者のデータ
CREATE TABLE IF NOT EXISTS admins (
  id serial PRIMARY KEY,
  name VARCHAR ( 50 ) UNIQUE NOT NULL,
  uid VARCHAR NOT NULL, 	                       -- uid from id platform
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  last_login TIMESTAMP
);

-- イベントのデータ
CREATE TABLE IF NOT EXISTS events (

  -- 公開用に欲しいもの

  id SERIAL PRIMARY KEY,
  slug VARCHAR,
  title VARCHAR NOT NULL,                        -- 名称
  body TEXT NOT NULL,                            -- 内容（マークダウン記法）
  genre VARCHAR,                                 -- ジャンル（パーティー、ライブ、セミナー等）
  tag VARCHAR,                                   -- タグ（樫原伸彦、デジハリ、ベトナム等）
  fee INTEGER,                                   -- イベントチャージ（入場料。未入力の場合は通常のBARのチャージ）
  ogp_img VARCHAR,                               -- アイキャッチ画像
  start_at TIMESTAMP,                          -- イベント開始日時
  end_at TIMESTAMP,                            -- イベント終了日時
  publish_at TIMESTAMP,                          -- 公開開始日時
  updated_at TIMESTAMP,                          -- 更新日時
  page_view INTEGER NOT NULL DEFAULT 0,          -- PV

  -- 管理用に欲しいもの

  creator_id INTEGER REFERENCES admins,          -- 作成者
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),   -- 作成日時
  published BOOLEAN NOT NULL DEFAULT 'f',        -- 公開フラグ
  memo VARCHAR                                   -- メモ
);

-- お問い合わせのデータ
CREATE TABLE IF NOT EXISTS contacts (

  -- 公開用に欲しいもの

  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,                        -- タイトル
  name VARCHAR NOT NULL,                         -- お名前
  email VARCHAR NOT NULL,                        -- メールアドレス
  phone VARCHAR,                                 -- 電話番号
  body TEXT NOT NULL,                            -- 内容

  -- 管理用に欲しいもの

  created_at TIMESTAMP NOT NULL DEFAULT NOW(),   -- 作成日時
  checked BOOLEAN NOT NULL DEFAULT 'f'           -- 確認済フラグ
);

