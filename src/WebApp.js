import { Routes, Route } from "react-router-dom"
import GlobalStyle from "GlobalTheme.js"
import ThemeManager from "Themes.js"
import Layout from "web/Layout.js"
import Info from "web/Info.js"
import Work from "web/Work.js"
import Blog from "web/Blog.js"
import Junk from "web/Junk.js"

const WebApp = () => {
  return (
    <ThemeManager>
      <GlobalStyle />
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route path="info" element={<Info />} />
          <Route path="work" element={<Work />} />
          <Route path="blog" element={<Blog />} />
          <Route path="junk" element={<Junk />} />
        </Route>
      </Routes>
    </ThemeManager>
  )
}

export default WebApp
