import React, { useEffect } from "react";
import * as test from "BevyTest";

const App = () => {
  useEffect(() => {
    console.log("TEST", test)
    test.default();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <canvas id="game" className="game" width={800} height={800} />
      </header>
    </div>
  );
}

export default App;
