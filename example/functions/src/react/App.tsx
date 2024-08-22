import logo from "./logo.svg";
import "./App.css";
import TodoList, { type Todo } from "./TodoList";

type Props = {
	todoItems?: Todo[];
	releaseName?: string;
};

function App({ todoItems = [], releaseName = "" }: Props) {
	return (
		<div class="app">
			<header class="header">
				<img src={logo} class="logo" alt="logo" />
			</header>
			<main class="main">
				<h1>LLRT React TODO - {releaseName}</h1>
				<TodoList items={todoItems} />
			</main>
		</div>
	);
}

export default App;
