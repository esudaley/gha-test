describe('test spec', () => {
  it('hits gui', () => {
    cy.visit("http://localhost:3000")
  })

  it('hits api', () => {
    cy.request("http://localhost:8000").then(response => 
        expect(response.body.message).to.equal("Hello World!"))
  })
})