module.exports = {
  pwa: {
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
  },
  transpileDependencies: [
    "vuetify",
  ],
}
