import GlassSurface from "@/components/reactbits/GlassSurface";

const App = () => {
	return (
		<main className="min-h-screen bg-background p-8">
			<div className="mx-auto max-w-4xl space-y-8">
				<div className="text-center space-y-4">
					<GlassSurface
						borderRadius={24}
						opacity={0.9}
						className="my-custom-class"
					>
						<h1 className="text-4xl font-bold tracking-tight">zims</h1>
						<p className="text-muted-foreground">pwm</p>
					</GlassSurface>
				</div>
			</div>
		</main>
	);
};

export default App;
