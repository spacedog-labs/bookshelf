// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const getBooks = () => {
    invoke("plugin:books|get_books")
      .then((resp) => {
        console.log(resp);
      })
      .catch((error) => console.error(error));
  };

  const addBook = () => {
    invoke("plugin:books|add_book", {
      book: { id: 124, title: "Test Book" },
    })
      .then((resp) => {
        console.log(resp);
      })
      .catch((error) => console.error(error));
  };

  const testBook = () => {
    invoke("plugin:books|test_book", {
      location: "/Users/colton/Desktop/herman-melville_moby-dick.epub",
    })
      .then((resp) => {
        console.log(resp);
      })
      .catch((error) => console.error(error));
  };

  return (
    <div className="App">
      <button onClick={getBooks}>get books</button>
      <button onClick={addBook}>add book</button>
      <button onClick={testBook}>test book</button>
    </div>
  );
}

export default App;
