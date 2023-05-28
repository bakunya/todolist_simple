import { useEffect, useState } from "react"

function App() {
  const [v, sv] = useState("")
  const [s, ss] = useState([])
  const rf = () => {
    ss([])
    fetch("/", { headers: { "content-type": "application/json" } }).then(r => r.json()).then(r => ss(r))
  }
  const os = (e) => {
    e.preventDefault()
    const fd = new FormData(e.target)
    fetch(`/`, { method: "post", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ task: fd.get("task") }) })
      .catch(e => console.log(e))
      .finally(() => {
        rf()
        sv("")
      })
  }

  useEffect(() => { rf() }, [])

  return (
    <>
      <h1>Todo List</h1>
      <form onSubmit={os}>
        <input type="text" name="task" value={v} onChange={e => sv(e.target.value)} />
        <button type="submit">Add</button>
      </form>

      <section>
        <br />
        {
          ss.length && s.map(v => <Item v={v} rf={rf} />)
        }
      </section>
    </>
  )
}

function Item({v, rf}) {
  const [t, sv] = useState(v.task)
  const [s, ss] = useState(true)
  const [c, sc] = useState(v.completed)
  const hb = () => {
    fetch(`/put?id=${v.id}`, { method: "put", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ task: t }) })
      .catch(e => console.log(e))
      .finally(() => {
        ss(true)
        rf()
      })
  }
  const oc = () => {
    let cc = c
    sc(!cc)
    fetch(`/put?id=${v.id}`, { method: "put", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ completed: !cc }) })
      .catch(e => console.log(e))
      .finally(() => rf())
  }  
  const od = () => {
    fetch(`/delete?id=${v.id}`, { method: "delete", headers: { "Content-Type": "application/json" } })
      .catch(e => console.log(e))
      .finally(() => rf())
  }
  const onc = (e) => sv(e.target.value)

  return (
    <section>
      {
        s ? <span onDoubleClick={() => ss(false)}>{t}</span> : <input onChange={onc} type="text" value={t} onBlur={hb} />
      }
      <input type="checkbox" checked={c} onChange={oc} />
      <button onClick={od}>Delete</button>
    </section>
  )
}

export default App
