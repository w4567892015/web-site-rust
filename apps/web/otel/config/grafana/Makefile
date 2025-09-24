MAKEFLAGS += --silent

GRAFANA_URL="http://localhost:3080/api"

DATASOURCE_DIR=./provisioning/datasources

import.datasources:
	for i in ${DATASOURCE_DIR}/*; do \
		curl -X "POST" "${GRAFANA_URL}/datasources" \
		-H "Content-Type: application/json" --data-binary @$$i ;\
	done

export.datasources:
	mkdir -p ${DATASOURCE_DIR} \
	&& curl -s "${GRAFANA_URL}/datasources" \
	| jq -c '.[]' \
	| while IFS= read -r line; do \
			type=$$(echo "$$line" | jq -r '.type') \
			uid=$$(echo "$$line" | jq -r '.uid') \
			&& echo "$$line" > "${DATASOURCE_DIR}/$$type-$$uid.json"; \
		done;
