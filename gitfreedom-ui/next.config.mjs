let userConfig = undefined;

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "export", // Gera uma versão estática da aplicação
  trailingSlash: true, // Adiciona uma barra no final das URLs
  eslint: {
    ignoreDuringBuilds: true, // Ignora erros do ESLint durante a build
  },
  typescript: {
    ignoreBuildErrors: true, // Ignora erros do TypeScript durante a build
  },
  images: {
    unoptimized: true, // Desativa a otimização de imagens (necessário para exportação estática)
  },
  experimental: {
    webpackBuildWorker: true, // Habilita workers para builds Webpack (melhora performance)
    parallelServerBuildTraces: true, // Habilita traces paralelos para builds do servidor
    parallelServerCompiles: true, // Habilita compilações paralelas para builds do servidor
  },
  // Configurações adicionais para otimizar a SPA
  optimizeFonts: true, // Otimiza o carregamento de fontes
  compress: true, // Habilita a compactação dos arquivos de saída
  productionBrowserSourceMaps: false, // Desativa source maps para o navegador em produção
};

// Função para mesclar configurações personalizadas
function mergeConfig(nextConfig, userConfig) {
  if (!userConfig) {
    return;
  }

  for (const key in userConfig) {
    if (
      typeof nextConfig[key] === 'object' &&
      !Array.isArray(nextConfig[key])
    ) {
      nextConfig[key] = {
        ...nextConfig[key],
        ...userConfig[key],
      };
    } else {
      nextConfig[key] = userConfig[key];
    }
  }
}

mergeConfig(nextConfig, userConfig);

export default nextConfig;