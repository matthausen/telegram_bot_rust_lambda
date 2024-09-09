BIN=app
VERSION=$(or ${CONFIG_APP_VERSION}, $(shell git tag --contains HEAD | tail -1))
C_APP_NAME=telegram_bot_lambda

all: run

# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## This help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-z0-9A-Z_-]+:.*?## / {printf "\033[36m%-45s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

watch: 
	cargo lambda watch

build:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

set-telegram-webhook:
	curl -F "url=https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/setWebhook?url=${API_GATEWAY_URL}" https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/setWebhook
	curl -F https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/setWebhook?url=https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/setWebhook?url=${API_GATEWAY_URL}

webhook-info:
	curl -s https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/getWebhookInfo

.PHONY: all help watch build deploy