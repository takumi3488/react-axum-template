import { useEffect, useState } from "react";

export function Main() {
	const [health, setHealth] = useState<string>("LOADING");
	useEffect(() => {
		async function checkHealth() {
			try {
				const response = await fetch("/api/health");
				const responseText = await response.text();
				setHealth(responseText);
			} catch (error) {
				console.error(error);
				setHealth("ERROR");
			}
		}
		const interval = setInterval(checkHealth, 1000);
		return () => clearInterval(interval);
	}, []);

	return (
		<div>
			<h1>reac-axum-template</h1>
			<p>backend: {health}</p>
		</div>
	);
}
