export default defineConfig({
  server: {
    proxy: {
      '/syncflow-api': {
        target: 'http://localhost:8081',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/syncflow-api/, ''),
        secure: false,
      },
    },
  },
});
