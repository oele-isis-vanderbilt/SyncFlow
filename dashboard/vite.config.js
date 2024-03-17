export default defineConfig({
  server: {
    proxy: {
      '/livekit-mmlaapi': {
        target: 'http://localhost:8081',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/livekit-mmlaapi/, ''),
        secure: false,
      },
    },
  },
});
