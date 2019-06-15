module.exports = {
  pwa: {
    workboxOptions: {
      runtimeCaching: [{
        // Match the exact path /settings.json.
        urlPattern: /^\/settings.json$/,
        // Apply a network-first strategy.
        handler: 'networkFirst',
        options: {
          // Fall back to the cache after 5 seconds.
          networkTimeoutSeconds: 5,
          // Use a custom cache name for this route.
          cacheName: 'manual-updates-cache',
          // Configure custom cache expiration.
          // expiration: {
          //   maxEntries: 5,
          //   maxAgeSeconds: 60,
          // },
          // Configure background sync.
          // backgroundSync: {
          //   name: 'manual-updates-queue',
          //   options: {
          //     maxRetentionTime: 60 * 60,
          //   },
          // },
          // Configure which responses are considered cacheable.
          cacheableResponse: {
            statuses: [0, 200],
            // headers: {'x-test': 'true'},
          },
          // Configure the broadcast cache update plugin.
          // broadcastUpdate: {
          //   channelName: 'my-update-channel',
          // },
          // Add in any additional plugin logic you need.
          // plugins: [
          //   {cacheDidUpdate: () => /* custom plugin code */}
          // ],
          // matchOptions and fetchOptions are used to configure the handler.
          // fetchOptions: {
          //   mode: 'no-cors',
          // },
          // matchOptions: {
          //   ignoreSearch: true,
          // },
        },
      }]
    }
  }
};
