module.exports = {
  pwa: {
    workboxOptions: {
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
