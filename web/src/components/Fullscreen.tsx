import React, { useEffect, useRef, useState } from 'react'


export function Fullscreen() {
    // console.log(OverlayDom)
    const containerRef = useRef(null)
    const [{ width, height }, setWidthHeight] = useState({ width: 0, height: 0 })
    useEffect(() => {
      const 
      setWidthHeight({
        width: containerRef.current.offsetWidth,
        height: containerRef.current!.offsetHeight,
      })
    }, [])
    return (
      <div
        ref={containerRef}
        id={"stage-container"}
        style={{
          width: width ? width : window.innerWidth,
          height: height ? height : window.innerHeight,
        }}
      >
        </div>
}