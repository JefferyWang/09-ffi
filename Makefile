build-js:
	@echo "Building JS"
	@cd node-binding && yarn && yarn build

run-js:
	@cd node-binding && node
