<template>
  <div id="about" class="lg:flex block lg:max-h-screen py-20 justify-between">
    <img class="lg:w-3/5 w-full" src="/assets/DSC08504.jpeg" />
    <div class="lg:w-2/5 w-full text-white text-center">
      <div class="text-4xl my-10 inline-block under">宮沢賢治</div>
      <div v-for="i in text" :key="i" class="mx-10">{{ i }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
const text = [
  "十日の月が西の煉瓦塀れんぐわべいにかくれるまで、もう一時間しかありませんでした。",
  "その青じろい月の明りを浴びて、獅子ししは檻をりのなかをのそのそあるいて居をりましたが、ほかのけだものどもは、頭をまげて前あしにのせたり、横にごろっとねころんだりしづかに睡ねむってゐました。夜中まで檻の中をうろうろうろうろしてゐた狐きつねさへ、をかしな顔をしてねむってゐるやうでした。",
  "わたくしは獅子の檻のところに戻って来て前のベンチにこしかけました。",
  "するとそこらがぼうっとけむりのやうになってわたくしもそのけむりだか月のあかりだかわからなくなってしまひました。",
  "いつのまにか獅子が立派な黒いフロックコートを着て、肩を張って立って",
  "「もうよからうな。」と云いひました。",
];
</script>

<style scoped>
.under {
  line-height: 3em;
  border-bottom: 1px solid #fff;
}
@media (max-width: 768px) {
}
</style>
