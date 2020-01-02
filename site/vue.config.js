module.exports = {
  pwa: {
    name: "Dextools",
    workboxPluginMode: 'InjectManifest',
    workboxOptions: {
      swSrc: './src/sw.js',
      swDest: 'service-worker.js',
      exclude: [
        /\.map$/,
        /img\/icons\//,
        /favicon\.ico$/,
        /^manifest.*\.js?$/,
        /^manifest.*\.json$/,
        /^settings\.json$/,
        /^CNAME$/,
      ],
    },
    iconPaths: {
      favicon32: 'img/icons/favicon-32x32.png',
      favicon16: 'img/icons/favicon-16x16.png',
      appleTouchIcon: 'img/icons/apple-touch-icon-180x180.png',
    }
  },
  transpileDependencies: [
    "vuetify",
  ],
}
