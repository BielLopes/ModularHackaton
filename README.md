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
Para executar seguir os passos:
- Inicialize o nitro devnode `./run-dev-node.sh`
- Faça deploy no node lolcal em localhost:8547 `cargo stylus deploy \
  --endpoint='http://localhost:8547' \
  --private-key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"`

### gitfreedom
Para executar seguir os passos:
- `cargo run -- --help`
- `cargo run -- config --public-key=<sua_chaave_publica>`
- `cargo run -- init`
- `cargo run -- share --private-key=<caminho_da_sua_chave_privada>`
- `cargo run -- clone gitfreedom-<sua_chave_publica>  `

