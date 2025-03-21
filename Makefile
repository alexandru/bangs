.PHONY: dist gen-bangs clean

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
