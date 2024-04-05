import React from "react";
import ReactDOM from "react-dom/client";

const root = ReactDOM.createRoot(document.querySelector("main"));
root.render(<App />);

function App() {
  const [data, setData] = React.useState(null);

  React.useEffect(() => {
    fetch("../report.json")
      .then((res) => res.json())
      .then(setData);
  }, []);

  if (!data) {
    return <>Loading ...</>;
  }

  return (
    <>
      <h1>
        Comparing <span className="gray">{data.old}</span> with{" "}
        <span className="gray">{data.new}</span>
      </h1>
      {data.changes.length === 0 && <>No changes</>}
      {data.changes.map((change) => (
        <Section key={change.name} change={change} />
      ))}
    </>
  );
}

function Section({ change }) {
  const [added, setAdded] = React.useState(null);
  const [removed, setRemoved] = React.useState(null);
  const [diff, setDiff] = React.useState(null);

  React.useEffect(() => {
    if (added && removed) {
      setDiff(visualDiff(added, removed));
    }
  }, [added, removed]);

  return (
    <section>
      <h2>{change.name}</h2>
      <div className="side-by-side">
        <Image
          className={"added"}
          title="Added"
          path={`../store/png/${change.name}-${change.added}.png`}
          detail={change.added}
          onLoad={setAdded}
        />
        <Image
          className={"removed"}
          title="Removed"
          path={`../store/png/${change.name}-${change.removed}.png`}
          detail={change.removed}
          onLoad={setRemoved}
        />
        {diff && (
          <>
            <div></div>
            <img
              className="diff"
              src={diff.toDataURL()}
              width={diff.width}
              height={diff.height}
            ></img>
          </>
        )}
      </div>
    </section>
  );
}

function Image({ className, title, path, detail, onLoad }) {
  const [size, setSize] = React.useState(undefined);
  React.useEffect(() => {
    loadCanvas(path).then((canvas) => {
      setSize([canvas.width, canvas.height]);
      onLoad(canvas);
    });
  }, [path]);

  return (
    <>
      <div>
        <h3 className={className}>{title}</h3>
        <div className="info">
          <span className="hash">{detail}</span>
          {size && (
            <span className="size">
              {size[0]} x {size[1]}
            </span>
          )}
        </div>
      </div>
      <img src={path} />
    </>
  );
}

async function loadCanvas(src) {
  const img = document.createElement("img");
  img.src = src;
  await new Promise((resolve) => (img.onload = resolve));

  const width = img.width;
  const height = img.height;

  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;

  const ctx = canvas.getContext("2d");
  ctx.drawImage(img, 0, 0);

  return canvas;
}

function visualDiff(a, b) {
  const width = Math.max(a.width, b.width);
  const height = Math.max(a.height, b.height);

  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;

  const ctx = canvas.getContext("2d");
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  const id = ctx.getImageData(0, 0, width, height);
  const idA = a.getContext("2d").getImageData(0, 0, a.width, a.height);
  const idB = b.getContext("2d").getImageData(0, 0, b.width, b.height);

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const i = (y * width + x) * 4;
      const iA = (y * a.width + x) * 4;
      const iB = (y * b.width + x) * 4;

      if (
        idA.data[iA] != idB.data[iB] ||
        idA.data[iA + 1] != idB.data[iB + 1] ||
        idA.data[iA + 2] != idB.data[iB + 2] ||
        idA.data[iA + 3] != idB.data[iB + 3]
      ) {
        id.data[i] = 255;
        id.data[i + 1] = 0;
        id.data[i + 2] = 0;
        id.data[i + 3] = 255;
      } else if (x < a.width && y < a.height) {
        id.data[i] = idA.data[iA];
        id.data[i + 1] = idA.data[iA + 1];
        id.data[i + 2] = idA.data[iA + 2];
        id.data[i + 3] = idA.data[iA + 3] / 3;
      }
    }
  }

  ctx.putImageData(id, 0, 0);

  return canvas;
}
