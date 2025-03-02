# ModularHackaton

Código para ser apreentado no Hackaton da Modular Crypto

## Descrição
São três repositório desenvolvidos para o Hackaton da Modular Crypto, sendo eles: 
- gitfreedom-ui -> Frontend em react para explorar os repositórios registrados on-chain
- gitfreedom-contract -> Smart contract para registrar repositórios usando Arbitrum Stylus SDK
- gitfreedom -> CLI tool para compartilhar e recuperar repositórios on-chain, é também responsável por toda a conexão peer to peer e por criar seeds e fazer download (clone) de repositórios (criando uma rede torrent)

### gitfreedom-ui
Para executar seguir os passos:
- `npm install --force && npm audit fix --force`
- `npm run dev`

### gitfreedom-contract
O contrato já se encontra na testnet da Arbitrum Sepolia, seu endereço on-chain 0x36AB833CfF7994F8a50E949f205aFD362BEEeF46 e a melhor rpc utilizada foi o https://rpc-sepolia.rockx.com/

### gitfreedom
Para executar seguir os passos:
- Instalar o Rust, execute o seguite comando no seu emulador de terminal favorito (ou em um  WSL): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Certifique-se de que o comando `cargo` está disponível no seu $PATH
- No diretório gitfreedom, execute o comando `cargo build` para gerar o binário. Obs: isso deve demorar um puco <br/>
- Para visualizar toda a interface de comandos disponíveis, execute: `cargo run -- --help`
- Como o gitfreedom é uma CLI que foi feita para ser executada e qualquer diretório, para uma melhor experiência, sugiro criar um alias para o binário, como: `alias gf="<path_desse_repositorio>/gitfreedom/target/debug/gf"`
- Para criar as configrações iniciais, execute: `gf config --public-key=<public_key>`, onde `<public_key>` é a chave pública na rede etherium que você deseja utilizar para compartilhar seus repositórios. é possível omitir o argumento `--public-key` se caso só queira clonar um repositorio.
- O proximo passo é prepara um repositório para ser compartilhado, para isso execute: `gf init`. Certifique-se de estar dentro do cwd de um repositório git, de ter configurado a chave pública e de que exista uma branch com nome 'main' no seu repositório.
- Para compartilhar o repositório, execute: `gf share --private-key=<pv_path>` onde `<pv_path>` é o caminho para a sua chave privada. A primeira coisa que vai ser feita é adicionar o hash manifesto do repositório no contrato, e em seguida, o gitfreedom vai copiar todos os arquivos em uma pasta temporária e vai adicionar todos os arquivos a uma rede torrent do repositório e começar a compartilhar.
- Para clonar um repositório, execute: `gf clone <name>-<public_key>`, onde `<name>` é o nome do repositório e `<public_key>` é a chave pública do dono do repositório que você deseja clonar (para fins de testes, é a sua chave pública). O gitfreedom vai reperar o hash do manifesto do contrato e em seguida baixa-lo, e com os dados desse arquivo vai poder baixar todos os arquivos do repositório da rede torrent do repositório. Ex: `gf clone gitfreedom-0x36AB833CfF7994F8a50E949f205aFD362BEEeF46`

