.PHONY: build dist gen-bangs clean

build:
	./gradlew build

gen-bangs:
	./gradlew generateBangs

dist:
	./gradlew jsBrowserDistribution

clean:
	./gradlew clean

run-dev:
	./gradlew jsBrowserDevelopmentRun -t

run-prod:
	./gradlew jsBrowserProductionRun

dependency-updates:
	./gradlew dependencyUpdates \
		-Drevision=release \
		-DoutputFormatter=html \
		--refresh-dependencies && \
		open build/dependencyUpdates/report.html

update-gradle:
	./gradlew wrapper --gradle-version latest
