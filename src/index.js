import React from "react"
import ReactDOM from "react-dom"
import "./index.css"
import { BrowserRouter as Router } from "react-router-dom"
import { isMobile } from "react-device-detect"
import WebApp from "WebApp.js"
import MobileApp from "MobileApp.js"

const content = isMobile ? <MobileApp /> : <WebApp />

ReactDOM.render(
  <React.StrictMode>
    <Router>{content}</Router>
  </React.StrictMode>,
  document.getElementById("root")
)
