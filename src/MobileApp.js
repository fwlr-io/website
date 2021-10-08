import { Routes, Route } from "react-router-dom"
import GlobalStyle from "GlobalTheme.js"
import ThemeManager from "Themes.js"
import Layout from "mob/Layout.js"
import Info from "mob/Info.js"
import Blog from "mob/Blog.js"

const MobileApp = () => {
  return (
    <ThemeManager>
      <GlobalStyle />
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route path="/" element={<Info />} />
          <Route path="blog" element={<Blog />} />
        </Route>
      </Routes>
    </ThemeManager>
  )
}

export default MobileApp

/*
<Route path="" element={}/>
*/
