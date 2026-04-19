# Guardian CLI

Cliente de linha de comando para o [Guardian Auth](https://guardian.whiskyrie.com.br). Permite gerenciar usuarios, roles e logs de auditoria diretamente do terminal.

## Requisitos

- Rust 1.85+ (edition 2024)
- Acesso a uma instancia do Guardian Auth (padrao: `https://guardian.whiskyrie.com.br`)

## Instalacao

```bash
git clone https://github.com/Whiskyrie/guardian_cli.git
cd guardian_cli
cargo build --release
```

O binario fica em `target/release/guardian`.

## Configuracao

O CLI procura a URL do Guardian Auth na seguinte ordem de prioridade:

1. Variavel de ambiente `GUARDIAN_URL` (pode ser definida em `.env` no diretorio atual)
2. Arquivo `~/.guardian/config.toml`
3. Valor padrao: `https://guardian.whiskyrie.com.br`

### Exemplo de config.toml

```toml
guardian_url = "https://guardian.whiskyrie.com.br"
```

### Exemplo de .env

```
GUARDIAN_URL=https://guardian.whiskyrie.com.br
```

O token de autenticacao e salvo em `~/.guardian/token` apos o login.

## Uso

### Autenticacao

```bash
# Login (senha sera pedida via prompt se nao informada)
guardian login --email admin@guardian.com --password 'senha'

# Login com prompt de senha
guardian login --email admin@guardian.com

# Ver usuario autenticado
guardian whoami

# Renovar token
guardian token

# Logout (remove o token salvo)
guardian logout
```

### Gerenciamento de usuarios (admin)

```bash
# Listar todos os usuarios
guardian user list

# Filtrar por role
guardian user list --role ADMIN

# Buscar por nome ou email
guardian user list --search admin

# Criar um usuario
guardian user create \
  --email novo@example.com \
  --password 'senha123' \
  --first Nome \
  --last Sobrenome

# Deletar um usuario
guardian user delete --id 2
```

### Gerenciamento de roles (admin)

```bash
# Atribuir roles a um usuario (separadas por virgula)
guardian role assign --user-id 2 --roles user,admin
```

Nota: o comando `role assign` substitui as roles do usuario, nao adiciona incrementalmente.

### Auditoria (admin)

```bash
# Listar logs de auditoria
guardian audit list

# Filtrar por acao
guardian audit list --action login

# Filtrar por usuario
guardian audit list --user-id 1

# Filtrar pelas ultimas N horas
guardian audit list --recent-hours 24
```

### Formato de saida

Por padrao, a saida e formatada em tabela. Use `--output json` para obter JSON:

```bash
guardian whoami --output json
guardian user list --output json
```

## Estrutura do projeto

```
src/
  main.rs          -- Entrypoint, dispatch de comandos
  error.rs         -- GuardianError (thiserror)
  config/mod.rs    -- Config: .env > ~/.guardian/config.toml > defaults
  api/
    mod.rs         -- GuardianClient (reqwest), graphql_request()
    models.rs      -- Structs serde espelhando a API GraphQL
    queries.rs     -- Constantes GraphQL para queries/mutations
  cli/
    mod.rs         -- clap Cli + Commands enum + --output flag
    login.rs       -- guardian login
    logout.rs      -- guardian logout
    whoami.rs      -- guardian whoami
    token.rs       -- guardian token (refresh)
    user.rs        -- guardian user list/create/delete
    role.rs        -- guardian role assign
    audit.rs       -- guardian audit list
  output/mod.rs    -- Formatacao pretty (comfy-table) e JSON
```

## Dependencias

| Crate        | Uso                                    |
|--------------|----------------------------------------|
| clap 4       | Parser de argumentos (derive)          |
| reqwest 0.12 | Cliente HTTP (rustls-tls, JSON)       |
| tokio 1      | Runtime async                          |
| serde 1      | Serializacao/deserializacao            |
| serde_json 1 | Manipulacao de JSON                     |
| toml 0.8     | Leitura de config.toml                 |
| dotenvy 0.15 | Carregamento de .env                   |
| comfy-table 7| Formatacao tabular                     |
| thiserror 2  | Tipos de erro customizados             |
| colored 3    | Colorizacao de saida                   |
| rpassword 7  | Prompt seguro de senha                 |

## Licenca

Projeto privado. Todos os direitos reservados.
